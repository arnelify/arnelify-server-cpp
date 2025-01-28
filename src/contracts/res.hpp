#ifndef ARNELIFY_SERVER_HANDLER_HPP
#define ARNELIFY_SERVER_HANDLER_HPP

#include <filesystem>
#include <functional>
#include <iostream>

#include "json.h"

using Req = Json::Value;

struct Res {
 private:
  Json::Value res;

 public:
  Res() {
    this->res["body"] = "";
    this->res["code"] = 200;
    this->res["filePath"] = "";
    this->res["headers"] = Json::objectValue;
  }

  void setCode(const int& code) { this->res["code"] = code; }
  void setFile(const std::string& filePath) { this->res["filePath"] = filePath; }
  void setHeader(const std::string& key, const std::string& value) {
    this->res["headers"][key] = value;
  }

  void addBody(const std::string& chunk) {
    this->res["body"] = this->res["body"].asString() + chunk;
  }

  void end() {
    if (!this->res["filePath"].asString().empty()) {
      this->res["body"] = "";
      return;
    }

    this->res["filePath"] = "";
  }

  const std::string serialize() {
    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    return Json::writeString(writer, this->res);
  }
};

struct StdToC {
  static std::function<void(const std::string&, const bool&)> callback;
  static std::function<void(const Req&, Res&)> handler;

  void setStdCallback(
      const std::function<void(const std::string&, const bool&)> callback) {
    StdToC::callback = callback;
  }

  void setStdHandler(const std::function<void(const Req&, Res&)> handler) {
    StdToC::handler = handler;
  }

  static void cCallback(const char* cMessage, const int isError) {
    callback(cMessage, isError);
  };

  static const char* cHandler(const char* cSerialized) {
    Json::Value req;
    Json::CharReaderBuilder reader;
    std::string errors;
    std::istringstream iss(cSerialized);
    if (!Json::parseFromStream(reader, iss, &req, &errors)) {
      std::cout
          << "[ArnelifyServer FFI]: Error in C++: cReq must be a valid JSON."
          << std::endl;
      exit(1);
    }

    Res res;
    handler(req, res);
    const std::string serialized = res.serialize();

    char* cRes = new char[serialized.size() + 1];
    std::copy(serialized.begin(), serialized.end(), cRes);
    cRes[serialized.size()] = '\0';
    return cRes;
  }
};

std::function<void(const std::string&, const bool&)> StdToC::callback = nullptr;
std::function<void(const Req&, Res&)> StdToC::handler = nullptr;

#endif