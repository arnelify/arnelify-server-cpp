#include <iostream>

#include "json.h"
#include "lib.hpp"

int main() {
  Http3Opts opts(
      /* allow_empty_files */ true,
      /* block_size_kb */ 64,
      /* cert_pem */ "certs/cert.pem",
      /* charset */ "utf-8",
      /* compression */ true,
      /* keep_alive */ 30,
      /* keep_extensions */ true,
      /* key_pem */ "certs/key.pem",
      /* max_fields*/ 60,
      /* max_fields_size_total_mb */ 60,
      /* max_files */ 1,
      /* max_files_size_total_mb */ 60,
      /* max_file_size_mb */ 60,
      /* port */ 4433,
      /* storage_path */ "/var/www/cpp/storage",
      /* thread_limit */ 4);

  Http3 http3(opts);
  Http3Logger http3_logger = [](const std::string& level,
                                const std::string& message) -> void {
    std::cout << "[Arnelify Server]: " << message << std::endl;
  };

  http3.logger(http3_logger);

  Http3Handler http3_handler = [](Http3Req& ctx, Http3Stream& stream) -> void {
    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    const std::string res = Json::writeString(writer, ctx);
    const Http1Res bytes(res.begin(), res.end());

    stream.set_code(200);
    stream.push_bytes(bytes);
    stream.end();
  };

  http3.on("/", http3_handler);
  http3.start();
}