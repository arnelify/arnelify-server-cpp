#ifndef ARNELIFY_SERVER_FFI_CPP
#define ARNELIFY_SERVER_FFI_CPP

#include "index.cpp"

extern "C" {

ArnelifyServer *server = nullptr;

void server_create(const char *cOpts) {
  Json::Value json;
  Json::CharReaderBuilder reader;
  std::string errors;

  std::istringstream iss(cOpts);
  if (!Json::parseFromStream(reader, iss, &json, &errors)) {
    std::cout << "[ArnelifyServer FFI]: Error in C: Invalid cOpts."
              << std::endl;
    exit(1);
  }

  const bool hasAllowEmptyFiles = json.isMember("SERVER_ALLOW_EMPTY_FILES") &&
                                  json["SERVER_ALLOW_EMPTY_FILES"].isBool();
  if (!hasAllowEmptyFiles) {
    std::cerr << "[Arnelify Server FFI]: Error in C: "
                 "'SERVER_ALLOW_EMPTY_FILES' is missing"
              << std::endl;
    exit(1);
  }

  const bool hasBlockSizeKb = json.isMember("SERVER_BLOCK_SIZE_KB") &&
                              json["SERVER_BLOCK_SIZE_KB"].isInt();
  if (!hasBlockSizeKb) {
    std::cerr << "[Arnelify Server FFI]: Error in C: "
                 "'SERVER_BLOCK_SIZE_KB' is missing"
              << std::endl;
    exit(1);
  }

  const bool hasCharset =
      json.isMember("SERVER_CHARSET") && json["SERVER_CHARSET"].isString();
  if (!hasCharset) {
    std::cerr << "[Arnelify Server FFI]: Error in C: "
                 "'SERVER_CHARSET' is missing"
              << std::endl;
    exit(1);
  }

  const bool hasGzip =
      json.isMember("SERVER_GZIP") && json["SERVER_GZIP"].isBool();
  if (!hasGzip) {
    std::cerr << "[Arnelify Server FFI]: Error in C: "
                 "'SERVER_GZIP' is missing"
              << std::endl;
    exit(1);
  }

  const bool hasKeepExtensions = json.isMember("SERVER_KEEP_EXTENSIONS") &&
                                 json["SERVER_KEEP_EXTENSIONS"].isBool();
  if (!hasKeepExtensions) {
    std::cerr << "[Arnelify Server FFI]: Error in C: "
                 "'SERVER_KEEP_EXTENSIONS' is missing"
              << std::endl;
    exit(1);
  }

  const bool hasMaxFields =
      json.isMember("SERVER_MAX_FIELDS") && json["SERVER_MAX_FIELDS"].isInt();
  if (!hasMaxFields) {
    std::cerr << "[Arnelify Server FFI]: Error in C: "
                 "'SERVER_MAX_FIELDS' is missing"
              << std::endl;
    exit(1);
  }

  const bool hasMaxFieldsSizeTotalMb =
      json.isMember("SERVER_MAX_FIELDS_SIZE_TOTAL_MB") &&
      json["SERVER_MAX_FIELDS_SIZE_TOTAL_MB"].isInt();
  if (!hasMaxFieldsSizeTotalMb) {
    std::cerr << "[Arnelify Server FFI]: Error in C: "
                 "'SERVER_MAX_FIELDS_SIZE_TOTAL_MB' is missing"
              << std::endl;
    exit(1);
  }

  const bool hasMaxFiles =
      json.isMember("SERVER_MAX_FILES") && json["SERVER_MAX_FILES"].isInt();
  if (!hasMaxFiles) {
    std::cerr << "[Arnelify Server FFI]: Error in C: "
                 "'SERVER_MAX_FILES' is missing"
              << std::endl;
    exit(1);
  }

  const bool hasMaxFilesSizeTotalMb =
      json.isMember("SERVER_MAX_FILES_SIZE_TOTAL_MB") &&
      json["SERVER_MAX_FILES_SIZE_TOTAL_MB"].isInt();
  if (!hasMaxFilesSizeTotalMb) {
    std::cerr << "[Arnelify Server FFI]: Error in C: "
                 "'SERVER_MAX_FILES_SIZE_TOTAL_MB' is missing"
              << std::endl;
    exit(1);
  }

  const bool hasMaxFileSizeMb = json.isMember("SERVER_MAX_FILE_SIZE_MB") &&
                                json["SERVER_MAX_FILE_SIZE_MB"].isInt();
  if (!hasMaxFileSizeMb) {
    std::cerr << "[Arnelify Server FFI]: Error in C: "
                 "'SERVER_MAX_FILE_SIZE_MB' is missing"
              << std::endl;
    exit(1);
  }

  const bool hasPort =
      json.isMember("SERVER_PORT") && json["SERVER_PORT"].isInt();
  if (!hasPort) {
    std::cerr << "[Arnelify Server FFI]: Error in C: "
                 "'SERVER_PORT' is missing"
              << std::endl;
    exit(1);
  }

  const bool hasQueueLimit =
      json.isMember("SERVER_QUEUE_LIMIT") && json["SERVER_QUEUE_LIMIT"].isInt();
  if (!hasQueueLimit) {
    std::cerr << "[Arnelify Server FFI]: Error in C: "
                 "'SERVER_QUEUE_LIMIT' is missing"
              << std::endl;
    exit(1);
  }

  const bool hasUploadDir = json.isMember("SERVER_UPLOAD_DIR") &&
                            json["SERVER_UPLOAD_DIR"].isString();
  if (!hasUploadDir) json["SERVER_UPLOAD_DIR"] = "./src/storage/upload";

  ArnelifyServerOpts opts(
      json["SERVER_ALLOW_EMPTY_FILES"].asBool(),
      json["SERVER_BLOCK_SIZE_KB"].asInt(), json["SERVER_CHARSET"].asString(),
      json["SERVER_GZIP"].asBool(), json["SERVER_KEEP_EXTENSIONS"].asBool(),
      json["SERVER_MAX_FIELDS"].asInt(),
      json["SERVER_MAX_FIELDS_SIZE_TOTAL_MB"].asInt(),
      json["SERVER_MAX_FILES"].asInt(),
      json["SERVER_MAX_FILES_SIZE_TOTAL_MB"].asInt(),
      json["SERVER_MAX_FILE_SIZE_MB"].asInt(), json["SERVER_PORT"].asInt(),
      json["SERVER_QUEUE_LIMIT"].asInt(), json["SERVER_UPLOAD_DIR"].asString());

  server = new ArnelifyServer(opts);
}

void server_destroy() { server = nullptr; }

void server_set_handler(const char *(*cHandler)(const char *),
                        const int hasRemove) {
  server->setHandler(
      [cHandler, hasRemove](const Req &req, Res res) -> void {
        Json::StreamWriterBuilder writer;
        writer["indentation"] = "";
        writer["emitUTF8"] = true;

        const std::string serialized = Json::writeString(writer, req);
        const char *cReq = serialized.c_str();
        const char *cRes = cHandler(cReq);
        if (cRes == nullptr) {
          std::cout
              << "[ArnelifyServer FFI]: Error in C: cRes must be a valid JSON."
              << std::endl;
          exit(1);
        }

        Json::Value deserialized;
        Json::CharReaderBuilder reader;
        std::string errors;
        std::istringstream iss(cRes);
        if (!Json::parseFromStream(reader, iss, &deserialized, &errors)) {
          std::cout
              << "[ArnelifyServer FFI]: Error in C: cRes must be a valid JSON."
              << std::endl;
          exit(1);
        }

        if (hasRemove == 1) delete[] cRes;
        const bool hasCode = deserialized.isMember("code");
        if (hasCode) {
          res->setCode(deserialized["code"].asInt());
        }

        if (deserialized.isMember("filePath") &&
            deserialized["filePath"].isString() &&
            !deserialized["filePath"].asString().empty()) {
          res->setFile(deserialized["filePath"].asString());
          res->end();
          return;
        }

        if (deserialized.isMember("body") && deserialized["body"].isString() &&
            !deserialized["body"].asString().empty()) {
          res->addBody(deserialized["body"].asString());
          res->end();
          return;
        }

        res->addBody("");
        res->end();
      });
}

void server_start(void (*cCallback)(const char *, const int)) {
  server->start([cCallback](const std::string &message, const bool &isError) {
    const char *cMessage = message.c_str();
    if (isError) {
      cCallback(cMessage, 1);
      return;
    }

    cCallback(cMessage, 0);
  });
}

void server_stop() { server->stop(); }
}

#endif