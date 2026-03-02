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

#ifndef ARNELIFY_SERVER_WT_HPP
#define ARNELIFY_SERVER_WT_HPP

#include <iostream>
#include <functional>
#include <unordered_map>
#include <mutex>
#include <sstream>
#include <optional>

#include "json.h"
#include "wt.h"

using WebTransportBytes = std::vector<std::uint8_t>;
using WebTransportCtx = Json::Value;
using WebTransportRes = Json::Value;

struct WebTransportOpts {
  const std::size_t block_size_kb;
  const std::string cert_pem;
  const bool compression;
  const std::uint64_t handshake_timeout;
  const std::string key_pem;
  const std::uint64_t max_message_size_kb;
  const std::uint64_t ping_timeout;
  const std::uint16_t port;
  const std::uint64_t send_timeout;
  const std::uint64_t thread_limit;

  WebTransportOpts(const std::size_t block_size_kb, const std::string& cert_pem,
                   const bool compression,
                   const std::uint64_t handshake_timeout,
                   const std::string& key_pem,
                   const std::uint64_t max_message_size_kb,
                   const std::uint64_t ping_timeout, const std::uint16_t port,
                   const std::uint64_t send_timeout,
                   const std::uint64_t thread_limit)
      : block_size_kb(block_size_kb),
        cert_pem(cert_pem),
        compression(compression),
        handshake_timeout(handshake_timeout),
        key_pem(key_pem),
        max_message_size_kb(max_message_size_kb),
        ping_timeout(ping_timeout),
        port(port),
        send_timeout(send_timeout),
        thread_limit(thread_limit) {}
};

class WebTransportStream {
 private:
  const int id;

 public:
  WebTransportStream(const int stream_id) : id(stream_id) {}
  ~WebTransportStream() {}

  void close() { wt_close(this->id); }

  void push(WebTransportRes payload, const WebTransportBytes& bytes) {
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

    wt_push(this->id, c_json.c_str(), c_bytes, c_bytes_len);
  }

  void push_bytes(const WebTransportBytes& bytes) {
    const char* c_bytes = nullptr;
    int c_bytes_len = 0;

    if (!bytes.empty()) {
      c_bytes = reinterpret_cast<const char*>(bytes.data());
      c_bytes_len = static_cast<int>(bytes.size());
    }

    wt_push_bytes(this->id, c_bytes, c_bytes_len);
  }

  void push_json(const WebTransportRes& payload) {
    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    const std::string c_json = Json::writeString(writer, payload);
    wt_push_json(this->id, c_json.c_str());
  }
};

using WebTransportLogger =
    std::function<void(const std::string&, const std::string&)>;
using WebTransportHandler = std::function<void(
    WebTransportCtx&, WebTransportBytes&, WebTransportStream&)>;

static std::unordered_map<int, WebTransportLogger> ARNELIFY_SERVER_WT_LOGGERS;
static std::mutex ARNELIFY_SERVER_WT_LOGGERS_MTX;

static std::unordered_map<int,
                          std::unordered_map<std::string, WebTransportHandler>>
    ARNELIFY_SERVER_WT_HANDLERS;
static std::mutex ARNELIFY_SERVER_WT_HANDLERS_MTX;

class WebTransport {
 private:
  int id;
  WebTransportOpts opts;

  static void logger_adapter(const int id, const char* c_level,
                             const char* c_message) {
    WebTransportLogger cb = [](const std::string&, const std::string& message) {
      std::cout << message << std::endl;
    };

    {
      std::lock_guard lock(ARNELIFY_SERVER_WT_LOGGERS_MTX);
      auto it = ARNELIFY_SERVER_WT_LOGGERS.find(id);
      if (it == ARNELIFY_SERVER_WT_LOGGERS.end()) return;
      cb = it->second;
    }

    if (!c_level || !c_message) return;
    cb(std::string(c_level), std::string(c_message));
  }

  static void handler_adapter(const int id, const int stream_id,
                              const char* c_ctx, const char* c_bytes,
                              const int c_bytes_len) {
    std::unordered_map<std::string, WebTransportHandler> handlers;
    {
      std::lock_guard lock(ARNELIFY_SERVER_WT_HANDLERS_MTX);
      auto it = ARNELIFY_SERVER_WT_HANDLERS.find(id);
      if (it == ARNELIFY_SERVER_WT_HANDLERS.end()) return;
      handlers = it->second;
    }

    Json::Value json;
    Json::CharReaderBuilder reader;
    std::string errs;
    std::stringstream ss(c_ctx);
    if (!parseFromStream(reader, ss, &json, &errs)) {
      std::cout << "[Arnelify Server]: C++ FFI error in wt_handler: Invalid "
                   "JSON in 'c_ctx'."
                << std::endl;
      exit(1);
    }

    const std::string topic = json["_state"]["topic"].asString();
    auto it = handlers.find(topic);
    if (it == handlers.end()) return;

    WebTransportBytes bytes;
    if (c_bytes && c_bytes_len > 0) {
      bytes.assign(c_bytes, c_bytes + c_bytes_len);
    }

    WebTransportStream stream(stream_id);
    it->second(json, bytes, stream);
  }

 public:
  explicit WebTransport(const WebTransportOpts& o) : id(0), opts(o) {
    Json::Value opts = Json::objectValue;

    opts["block_size_kb"] = static_cast<Json::UInt64>(this->opts.block_size_kb);
    opts["cert_pem"] = this->opts.cert_pem;
    opts["compression"] = this->opts.compression;
    opts["handshake_timeout"] =
        static_cast<Json::UInt64>(this->opts.handshake_timeout);
    opts["key_pem"] = this->opts.key_pem;
    opts["max_message_size_kb"] =
        static_cast<Json::UInt64>(this->opts.max_message_size_kb);
    opts["ping_timeout"] = static_cast<Json::UInt64>(this->opts.ping_timeout);
    opts["port"] = static_cast<Json::UInt>(this->opts.port);
    opts["send_timeout"] = static_cast<Json::UInt64>(this->opts.send_timeout);
    opts["thread_limit"] = static_cast<Json::UInt64>(this->opts.thread_limit);

    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    this->id = wt_create(Json::writeString(writer, opts).c_str());
  }

  ~WebTransport() {
    {
      std::lock_guard lock(ARNELIFY_SERVER_WT_LOGGERS_MTX);
      ARNELIFY_SERVER_WT_LOGGERS.erase(id);
    }

    {
      std::lock_guard lock(ARNELIFY_SERVER_WT_HANDLERS_MTX);
      ARNELIFY_SERVER_WT_HANDLERS.erase(id);
    }

    wt_destroy(id);
  }

  void logger(const WebTransportLogger& cb) {
    {
      std::lock_guard lock(ARNELIFY_SERVER_WT_LOGGERS_MTX);
      ARNELIFY_SERVER_WT_LOGGERS[id] = cb;
    }
    wt_logger(id, &WebTransport::logger_adapter);
  }

  void on(const std::string& path, const WebTransportHandler& cb) {
    {
      std::lock_guard lock(ARNELIFY_SERVER_WT_HANDLERS_MTX);
      ARNELIFY_SERVER_WT_HANDLERS[id][path] = cb;
    }
    wt_on(id, path.c_str(), &WebTransport::handler_adapter);
  }

  void start() { wt_start(id); }
  void stop() { wt_stop(id); }
};

#endif