#include <iostream>

#include "json.h"
#include "lib.hpp"

int main() {
  Http1Opts opts(
      /* allow_empty_files */ true,
      /* block_size_kb */ 64,
      /* charset */ "utf-8",
      /* compression */ true,
      /* keep_alive */ 30,
      /* keep_extensions */ true,
      /* max_fields*/ 60,
      /* max_fields_size_total_mb */ 1,
      /* max_files */ 1,
      /* max_files_size_total_mb */ 60,
      /* max_file_size_mb */ 60,
      /* port */ 4433,
      /* storage_path */ "/var/www/cpp/storage",
      /* thread_limit */ 4);

  Http1 http1(opts);
  Http1Logger http1_logger = [](const std::string& _level,
                                const std::string& message) -> void {
    std::cout << "[Arnelify Server]: " << message << std::endl;
  };

  http1.logger(http1_logger);
  Http1Handler http1_handler = [](Http1Req& ctx, Http1Stream& stream) -> void {
    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    const std::string res = Json::writeString(writer, ctx);
    const Http1Res bytes(res.begin(), res.end());

    stream.set_code(200);
    stream.push_bytes(bytes, false);
    stream.end();
  };

  http1.on("/", http1_handler);
  http1.start();
}