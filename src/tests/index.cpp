#ifndef ARNELIFY_SERVER_TEST_CPP
#define ARNELIFY_SERVER_TEST_CPP

#include <iostream>

#include "json.h"

#include "../index.h"

int main(int argc, char* argv[]) {
  Http1Opts http1Opts(
      /* SERVER_ALLOW_EMPTY_FILES */ true,
      /* SERVER_BLOCK_SIZE_KB */ 64,
      /* SERVER_CHARSET */ "UTF-8",
      /* SERVER_GZIP */ true,
      /* SERVER_KEEP_EXTENSIONS */ true,
      /* SERVER_MAX_FIELDS */ 1024,
      /* SERVER_MAX_FIELDS_SIZE_TOTAL_MB */ 20,
      /* SERVER_MAX_FILES */ 1,
      /* SERVER_MAX_FILES_SIZE_TOTAL_MB */ 60,
      /* SERVER_MAX_FILE_SIZE_MB */ 60,
      /* SERVER_NET_CHECK_FREQ_MS */ 50,
      /* SERVER_PORT */ 3001,
      /* SERVER_THREAD_LIMIT */ 5,
      /* SERVER_QUEUE_LIMIT */ 1024,
      /* SERVER_UPLOAD_DIR */ "./storage/upload");

  Http1 http1(http1Opts);
  http1.handler([](const Http1Req& req, Http1Res res) {
    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    res->setCode(200);
    res->addBody(Json::writeString(writer, req));
    res->end();
  });

  http1.start([](const std::string& message, const bool& isError) {
    if (isError) {
      std::cout << "[Arnelify Server]: Error: " << message << std::endl;
      exit(1);
    }

    std::cout << "[Arnelify Server]: " << message << std::endl;
  });

  return 0;
}

#endif