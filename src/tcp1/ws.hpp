// MIT LICENSE
//
// COPYRIGHT (R) 2025 ARNELIFY. AUTHOR: TARON SARKISYAN
//
// PERMISSION IS HEREBY GRANTED, FREE OF CHARGE, TO ANY PERSON OBTAINING A COPY
// OF THIS SOFTWARE AND ASSOCIATED DOCUMENTATION FILES (THE "SOFTWARE"), TO DEAL
// IN THE SOFTWARE WITHOUT RESTRICTION, INCLUDING WITHOUT LIMITATION THE RIGHTS
// TO USE, COPY, MODIFY, MERGE, PUBLISH, DISTRIBUTE, SUBLICENSE, AND/OR SELL
// COPIES OF THE SOFTWARE, AND TO PERMIT PERSONS TO WHOM THE SOFTWARE IS
// FURNISHED TO DO SO, SUBJECT TO THE FOLLOWING CONDITIONS:
//
// THE ABOVE COPYRIGHT NOTICE AND THIS PERMISSION NOTICE SHALL BE INCLUDED IN ALL
// COPIES OR SUBSTANTIAL PORTIONS OF THE SOFTWARE.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#ifndef ARNELIFY_SERVER_WS_HPP
#define ARNELIFY_SERVER_WS_HPP

#include <iostream>
#include <functional>
#include <unordered_map>
#include <mutex>
#include <sstream>
#include <optional>

#include "json.h"
#include "ws.h"

using WebSocketBytes = std::vector<std::uint8_t>;
using WebSocketCtx = Json::Value;
using WebSocketRes = Json::Value;

struct WebSocketOpts {
  const std::size_t block_size_kb;
  const bool compression;
  const std::uint64_t handshake_timeout;
  const std::uint64_t max_message_size_kb;
  const std::uint64_t ping_timeout;
  const std::uint16_t port;
  const std::uint64_t send_timeout;
  const std::uint64_t thread_limit;

  WebSocketOpts(const std::size_t block_size_kb, const bool compression,
                const std::uint64_t handshake_timeout,
                const std::uint64_t max_message_size_kb,
                const std::uint64_t ping_timeout, const std::uint16_t port,
                const std::uint64_t send_timeout,
                const std::uint64_t thread_limit)
      : block_size_kb(block_size_kb),
        compression(compression),
        handshake_timeout(handshake_timeout),
        max_message_size_kb(max_message_size_kb),
        ping_timeout(ping_timeout),
        port(port),
        send_timeout(send_timeout),
        thread_limit(thread_limit) {}
};

class WebSocketStream {
 private:
  const int id;

 public:
  WebSocketStream(const int stream_id) : id(stream_id) {}
  ~WebSocketStream() {}

  void close() { ws_close(this->id); }

  void push(WebSocketRes payload, const WebSocketBytes& bytes) {
    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    const std::string c_json = Json::writeString(writer, payload);
    const char* c_bytes = nullptr;
    int c_bytes_len = 0;

    if (!bytes.empty()) {
      c_bytes = reinterpret_cast<const char*>(bytes.data());
      c_bytes_len = static_cast<int>(bytes.size());
    }

    ws_push(this->id, c_json.c_str(), c_bytes, c_bytes_len);
  }

  void push_bytes(const WebSocketBytes& bytes) {
    const char* c_bytes = nullptr;
    int c_bytes_len = 0;

    if (!bytes.empty()) {
      c_bytes = reinterpret_cast<const char*>(bytes.data());
      c_bytes_len = static_cast<int>(bytes.size());
    }

    ws_push_bytes(this->id, c_bytes, c_bytes_len);
  }

  void push_json(const WebSocketRes& payload) {
    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    const std::string c_json = Json::writeString(writer, payload);
    ws_push_json(this->id, c_json.c_str());
  }
};

using WebSocketLogger =
    std::function<void(const std::string&, const std::string&)>;
using WebSocketHandler =
    std::function<void(WebSocketCtx&, WebSocketBytes&, WebSocketStream&)>;

static std::unordered_map<int, WebSocketLogger> ARNELIFY_SERVER_WS_LOGGERS;
static std::mutex ARNELIFY_SERVER_WS_LOGGERS_MTX;

static std::unordered_map<int,
                          std::unordered_map<std::string, WebSocketHandler>>
    ARNELIFY_SERVER_WS_HANDLERS;
static std::mutex ARNELIFY_SERVER_WS_HANDLERS_MTX;

class WebSocket {
 private:
  int id;
  WebSocketOpts opts;

  static void logger_adapter(const int id, const char* c_level,
                             const char* c_message) {
    WebSocketLogger cb = [](const std::string&, const std::string& message) {
      std::cout << message << std::endl;
    };

    {
      std::lock_guard lock(ARNELIFY_SERVER_WS_LOGGERS_MTX);
      auto it = ARNELIFY_SERVER_WS_LOGGERS.find(id);
      if (it == ARNELIFY_SERVER_WS_LOGGERS.end()) return;
      cb = it->second;
    }

    if (!c_level || !c_message) return;
    cb(std::string(c_level), std::string(c_message));
  }

  static void handler_adapter(const int id, const int stream_id,
                              const char* c_ctx, const char* c_bytes,
                              const int c_bytes_len) {
    std::unordered_map<std::string, WebSocketHandler> handlers;
    {
      std::lock_guard lock(ARNELIFY_SERVER_WS_HANDLERS_MTX);
      auto it = ARNELIFY_SERVER_WS_HANDLERS.find(id);
      if (it == ARNELIFY_SERVER_WS_HANDLERS.end()) return;
      handlers = it->second;
    }

    Json::Value json;
    Json::CharReaderBuilder reader;
    std::string errs;
    std::stringstream ss(c_ctx);
    if (!parseFromStream(reader, ss, &json, &errs)) {
      std::cout << "[Arnelify Server]: C++ FFI error in ws_handler: Invalid "
                   "JSON in 'c_ctx'."
                << std::endl;
      exit(1);
    }

    const std::string topic = json["_state"]["topic"].asString();
    auto it = handlers.find(topic);
    if (it == handlers.end()) return;

    WebSocketBytes bytes;
    if (c_bytes && c_bytes_len > 0) {
      bytes.assign(c_bytes, c_bytes + c_bytes_len);
    }

    WebSocketStream stream(stream_id);
    it->second(json, bytes, stream);
  }

 public:
  explicit WebSocket(const WebSocketOpts& o) : id(0), opts(o) {
    Json::Value opts = Json::objectValue;

    opts["block_size_kb"] = static_cast<Json::UInt64>(this->opts.block_size_kb);
    opts["compression"] = this->opts.compression;
    opts["handshake_timeout"] =
        static_cast<Json::UInt64>(this->opts.handshake_timeout);
    opts["max_message_size_kb"] =
        static_cast<Json::UInt64>(this->opts.max_message_size_kb);
    opts["ping_timeout"] = static_cast<Json::UInt64>(this->opts.ping_timeout);
    opts["port"] = static_cast<Json::UInt>(this->opts.port);
    opts["send_timeout"] = static_cast<Json::UInt64>(this->opts.send_timeout);
    opts["thread_limit"] = static_cast<Json::UInt64>(this->opts.thread_limit);

    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    this->id = ws_create(Json::writeString(writer, opts).c_str());
  }

  ~WebSocket() {
    {
      std::lock_guard lock(ARNELIFY_SERVER_WS_LOGGERS_MTX);
      ARNELIFY_SERVER_WS_LOGGERS.erase(id);
    }

    {
      std::lock_guard lock(ARNELIFY_SERVER_WS_HANDLERS_MTX);
      ARNELIFY_SERVER_WS_HANDLERS.erase(id);
    }

    ws_destroy(id);
  }

  void logger(const WebSocketLogger& cb) {
    {
      std::lock_guard lock(ARNELIFY_SERVER_WS_LOGGERS_MTX);
      ARNELIFY_SERVER_WS_LOGGERS[id] = cb;
    }
    ws_logger(id, &WebSocket::logger_adapter);
  }

  void on(const std::string& path, const WebSocketHandler& cb) {
    {
      std::lock_guard lock(ARNELIFY_SERVER_WS_HANDLERS_MTX);
      ARNELIFY_SERVER_WS_HANDLERS[id][path] = cb;
    }
    ws_on(id, path.c_str(), &WebSocket::handler_adapter);
  }

  void start() { ws_start(id); }
  void stop() { ws_stop(id); }
};

#endif