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

#ifndef ARNELIFY_SERVER_HTTP3_HPP
#define ARNELIFY_SERVER_HTTP3_HPP

#include <iostream>
#include <functional>
#include <unordered_map>
#include <mutex>
#include <sstream>
#include <optional>

#include "json.h"
#include "http3.h"

using Http3Ctx = Json::Value;
using Http3Req = Json::Value;
using Http3Res = std::vector<std::uint8_t>;

struct Http3Opts {
 public:
  const bool allow_empty_files;
  const std::size_t block_size_kb;
  const std::string cert_pem;
  const std::string charset;
  const bool compression;
  const std::uint8_t keep_alive;
  const bool keep_extensions;
  const std::string key_pem;
  const std::uint32_t max_fields;
  const std::size_t max_fields_size_total_mb;
  const std::uint32_t max_files;
  const std::size_t max_files_size_total_mb;
  const std::size_t max_file_size_mb;
  const std::uint16_t port;
  const std::string storage_path;
  const std::uint64_t thread_limit;

  Http3Opts(bool allow_empty_files, const std::size_t block_size_kb,
            const std::string& cert_pem, const std::string& charset,
            const bool compression, const std::uint8_t keep_alive,
            const bool keep_extensions, const std::string& key_pem,
            const std::uint32_t max_fields,
            const std::size_t max_fields_size_total_mb,
            const std::uint32_t max_files,
            const std::size_t max_files_size_total_mb,
            const std::size_t max_file_size_mb, const std::uint16_t port,
            const std::string& storage_path, const std::uint64_t thread_limit)
      : allow_empty_files(allow_empty_files),
        block_size_kb(block_size_kb),
        cert_pem(cert_pem),
        charset(charset),
        compression(compression),
        keep_alive(keep_alive),
        keep_extensions(keep_extensions),
        key_pem(key_pem),
        max_fields(max_fields),
        max_fields_size_total_mb(max_fields_size_total_mb),
        max_files(max_files),
        max_files_size_total_mb(max_files_size_total_mb),
        max_file_size_mb(max_file_size_mb),
        port(port),
        storage_path(storage_path),
        thread_limit(thread_limit) {}
};

class Http3Stream {
 private:
  const int id;

 public:
  Http3Stream(const int stream_id) : id(stream_id) {}
  ~Http3Stream() {}

  void add_header(const std::string& key, const std::string& value) {
    http3_add_header(this->id, key.c_str(), value.c_str());
  }

  void end() { http3_end(this->id); }

  void push_bytes(const std::vector<std::uint8_t>& bytes,
                  const bool is_attachment = false) {
    const char* c_bytes = nullptr;
    int c_bytes_len = 0;

    if (!bytes.empty()) {
      c_bytes = reinterpret_cast<const char*>(bytes.data());
      c_bytes_len = static_cast<int>(bytes.size());
    }

    http3_push_bytes(this->id, c_bytes, c_bytes_len, is_attachment ? 1 : 0);
  }

  void push_file(const std::string& file_path,
                 const bool is_attachment = false) {
    http3_push_file(this->id, file_path.c_str(), is_attachment ? 1 : 0);
  }

  void push_json(Json::Value json, const bool is_attachment = true) {
    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    const std::string c_json = Json::writeString(writer, json);
    http3_push_json(this->id, c_json.c_str(), is_attachment ? 1 : 0);
  }

  void set_code(const int code) { http3_set_code(this->id, code); }

  void set_compression(const std::optional<std::string>& compression) {
    if (compression.has_value()) {
      http3_set_compression(this->id, compression->c_str());
    } else {
      http3_set_compression(this->id, nullptr);
    }
  }

  void set_headers(
      const std::vector<std::pair<std::string, std::string>>& headers) {
    Json::Value json = Json::arrayValue;

    for (const auto& kv : headers) {
      Json::Value obj(Json::objectValue);
      obj[kv.first] = kv.second;
      json.append(obj);
    }

    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    const std::string c_json = Json::writeString(writer, json);
    http3_set_headers(this->id, c_json.c_str());
  }
};

using Http3Logger = std::function<void(const std::string&, const std::string&)>;
using Http3Handler = std::function<void(Http3Req&, Http3Stream&)>;

static std::unordered_map<int, Http3Logger> ARNELIFY_SERVER_HTTP3_LOGGERS;
static std::mutex ARNELIFY_SERVER_HTTP3_LOGGERS_MTX;

static std::unordered_map<int, std::unordered_map<std::string, Http3Handler>>
    ARNELIFY_SERVER_HTTP3_HANDLERS;
static std::mutex ARNELIFY_SERVER_HTTP3_HANDLERS_MTX;

class Http3 {
 private:
  int id;
  Http3Opts opts;

  static void logger_adapter(const int id, const char* c_level,
                             const char* c_message) {
    Http3Logger cb = [](const std::string&, const std::string& message) {
      std::cout << message << std::endl;
    };

    {
      std::lock_guard lock(ARNELIFY_SERVER_HTTP3_LOGGERS_MTX);
      auto it = ARNELIFY_SERVER_HTTP3_LOGGERS.find(id);
      if (it == ARNELIFY_SERVER_HTTP3_LOGGERS.end()) return;
      cb = it->second;
    }

    if (!c_level || !c_message) return;
    cb(std::string(c_level), std::string(c_message));
  }

  static void handler_adapter(const int id, const int stream_id,
                              const char* c_ctx) {
    std::unordered_map<std::string, Http3Handler> handlers;
    {
      std::lock_guard lock(ARNELIFY_SERVER_HTTP3_HANDLERS_MTX);
      auto it = ARNELIFY_SERVER_HTTP3_HANDLERS.find(id);
      if (it == ARNELIFY_SERVER_HTTP3_HANDLERS.end()) return;
      handlers = it->second;
    }

    Json::Value json;
    Json::CharReaderBuilder reader;
    std::string errs;
    std::stringstream ss(c_ctx);
    if (!parseFromStream(reader, ss, &json, &errs)) {
      std::cout << "[Arnelify Server]: C++ FFI error in http3_handler: Invalid "
                   "JSON in 'c_ctx'."
                << std::endl;
      exit(1);
    }

    const std::string path = json["_state"]["path"].asString();
    auto it = handlers.find(path);
    if (it == handlers.end()) return;

    Http3Stream stream(stream_id);
    it->second(json, stream);
  }

 public:
  explicit Http3(const Http3Opts& o) : id(0), opts(o) {
    Json::Value opts = Json::objectValue;

    opts["allow_empty_files"] = this->opts.allow_empty_files;
    opts["block_size_kb"] = static_cast<Json::UInt64>(this->opts.block_size_kb);
    opts["cert_pem"] = this->opts.cert_pem;
    opts["charset"] = this->opts.charset;
    opts["compression"] = this->opts.compression;
    opts["keep_alive"] = this->opts.keep_alive;
    opts["keep_extensions"] = this->opts.keep_extensions;
    opts["key_pem"] = this->opts.key_pem;
    opts["max_fields"] = this->opts.max_fields;
    opts["max_fields_size_total_mb"] =
        static_cast<Json::UInt64>(this->opts.max_fields_size_total_mb);
    opts["max_files"] = this->opts.max_files;
    opts["max_files_size_total_mb"] =
        static_cast<Json::UInt64>(this->opts.max_files_size_total_mb);
    opts["max_file_size_mb"] =
        static_cast<Json::UInt64>(this->opts.max_file_size_mb);
    opts["port"] = this->opts.port;
    opts["storage_path"] = this->opts.storage_path;
    opts["thread_limit"] = static_cast<Json::UInt64>(this->opts.thread_limit);

    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    this->id = http3_create(Json::writeString(writer, opts).c_str());
  }

  ~Http3() {
    {
      std::lock_guard lock(ARNELIFY_SERVER_HTTP3_LOGGERS_MTX);
      ARNELIFY_SERVER_HTTP3_LOGGERS.erase(id);
    }

    {
      std::lock_guard lock(ARNELIFY_SERVER_HTTP3_HANDLERS_MTX);
      ARNELIFY_SERVER_HTTP3_HANDLERS.erase(id);
    }

    http3_destroy(id);
  }

  void logger(const Http3Logger& cb) {
    {
      std::lock_guard lock(ARNELIFY_SERVER_HTTP3_LOGGERS_MTX);
      ARNELIFY_SERVER_HTTP3_LOGGERS[id] = cb;
    }
    http3_logger(id, &Http3::logger_adapter);
  }

  void on(const std::string& path, const Http3Handler& cb) {
    {
      std::lock_guard lock(ARNELIFY_SERVER_HTTP3_HANDLERS_MTX);
      ARNELIFY_SERVER_HTTP3_HANDLERS[id][path] = cb;
    }
    http3_on(id, path.c_str(), &Http3::handler_adapter);
  }

  void start() { http3_start(id); }
  void stop() { http3_stop(id); }
};

#endif