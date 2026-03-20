#include <iostream>

#include "json.h"
#include "lib.hpp"

int main() {
  Http2Opts opts(
      /* allow_empty_files */ true,
      /* block_size_kb */ 64,
      /* cert_pem */ "certs/cert.pem",
      /* charset */ "utf-8",
      /* compression */ true,
      /* keep_alive */ 30,
      /* keep_extensions */ true,
      /* key_pem */ "certs/key.pem",
      /* max_fields*/ 60,
      /* max_fields_size_total_mb */ 1,
      /* max_files */ 1,
      /* max_files_size_total_mb */ 60,
      /* max_file_size_mb */ 60,
      /* port */ 4433,
      /* storage_path */ "/var/www/cpp/storage",
      /* thread_limit */ 4);

  Http2 http2(opts);
  Http2Logger http2_logger = [](const std::string& _level,
                                const std::string& message) -> void {
    std::cout << "[Arnelify Server]: " << message << std::endl;
  };

  http2.logger(http2_logger);
  Http2Handler http2_handler = [](Http2Req& ctx, Http2Stream& stream) -> void {
    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    const std::string res = Json::writeString(writer, ctx);
    const Http1Res bytes(res.begin(), res.end());

    stream.set_code(200);
    stream.push_bytes(bytes, false);
    stream.end();
  };

  http2.on("/", http2_handler);
  http2.start();
}