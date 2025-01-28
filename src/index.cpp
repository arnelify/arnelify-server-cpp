#ifndef ARNELIFY_SERVER_CPP
#define ARNELIFY_SERVER_CPP

#include <dlfcn.h>
#include <string>
#include <filesystem>
#include <functional>
#include <iostream>
#include <stdexcept>
#include <vector>

#include "json.h"

#include "contracts/res.hpp"

class ArnelifyServer {
 private:
  void *lib = nullptr;
  std::filesystem::path libPath;
  const Json::Value opts;
  StdToC stdtoc;

  void (*server_create)(const char *);
  void (*server_destroy)();
  void (*server_set_handler)(const char *(*)(const char *), const int);
  void (*server_start)(void (*)(const char *, const int));
  void (*server_stop)();

  template <typename T>
  void loadFunction(const std::string &name, T &func) {
    func = reinterpret_cast<T>(dlsym(this->lib, name.c_str()));
    if (!func) {
      throw std::runtime_error(dlerror());
    }
  }

  std::function<void(const Req &, Res &)> handler = [](const Req &req,
                                                       Res &res) {
    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;

    Json::Value json;
    json["code"] = 200;
    json["success"] = "Welcome to Arnelify Server";
    res.addBody(Json::writeString(writer, json));
    res.end();
  };

 public:
  ArnelifyServer(const Json::Value &o) : opts(o) {
    std::filesystem::path folderPath = std::filesystem::current_path();
    this->libPath = folderPath / "build" / "index.so";
    this->lib = dlopen(this->libPath.c_str(), RTLD_LAZY);
    if (!this->lib) throw std::runtime_error(dlerror());

    loadFunction("server_create", this->server_create);
    loadFunction("server_destroy", this->server_destroy);
    loadFunction("server_set_handler", this->server_set_handler);
    loadFunction("server_start", this->server_start);
    loadFunction("server_stop", this->server_stop);

    Json::StreamWriterBuilder writer;
    writer["indentation"] = "";
    writer["emitUTF8"] = true;
    
    const std::string cOpts = Json::writeString(writer, this->opts);
    this->server_create(cOpts.c_str());
  }

  ~ArnelifyServer() {
    if (!this->lib) return;
    this->server_destroy();
    dlclose(this->lib);
    this->lib = nullptr;
  }

  void setHandler(const std::function<void(const Req &, Res &)> &handler) {
    stdtoc.setStdHandler(handler);
    this->server_set_handler(StdToC::cHandler, 1);
  }

  void start(
      const std::function<void(const std::string &, const bool &)> &callback) {
    stdtoc.setStdCallback(callback);
    this->server_start(StdToC::cCallback);
  }

  void stop() { this->server_stop(); }
};

#endif