<img src="https://static.wikia.nocookie.net/arnelify/images/c/c8/Arnelify-logo-2024.png/revision/latest?cb=20240701012515" style="width:336px;" alt="Arnelify Logo" />

![Arnelify Server for C++](https://img.shields.io/badge/Arnelify%20Server%20for%20C++-0.9.3-yellow) ![C++](https://img.shields.io/badge/C++-2b-red) ![G++](https://img.shields.io/badge/G++-15.2.0-blue) ![C-Lang](https://img.shields.io/badge/CLang-19.1.7-blue)

## 🚀 About

**Arnelify® Server for C & C++** — a multi-language server with HTTP 3.0 and WebTransport support.

All supported protocols:
| **#** | **Protocol** | **Transport** |
| - | - | - |
| 1 | TCP1 | HTTP 1.1 |
| 2 | TCP1 | HTTP 2.0 |
| 3 | TCP1 | WebSocket |
| 4 | TCP2 | HTTP 3.0 |
| 5 | TCP2 | WebTransport |

## 📋 Minimal Requirements
> Important: It's strongly recommended to use in a container that has been built from the gcc v15.2.0 image.
* CPU: Apple M1 / Intel Core i7 / AMD Ryzen 7
* OS: Debian 11 / MacOS 15 / Windows 10 with <a href="https://learn.microsoft.com/en-us/windows/wsl/install">WSL2</a>.
* RAM: 4 GB

## 📦 Installation
Run in terminal:
```bash
git clone git@github.com:arnelify/arnelify-server-cpp.git
```
## 🎉 TCP2 / WebTransport

### 📚 Configuration

| **Option** | **Description** |
| - | - |
| **BLOCK_SIZE_KB**| The size of the allocated memory used for processing large packets. |
| **CERT_PEM**| Path to the TLS cert-file in PEM format. |
| **COMPRESSION**| If this option is enabled, the server will use BROTLI compression if the client application supports it. This setting increases CPU resource consumption. The server will not use compression if the data size exceeds the value of **BLOCK_SIZE_KB**. |
| **HANDSHAKE_TIMEOUT**| Maximum time in seconds to complete the TLS handshake. |
| **KEY_PEM**| Path to the TLS private key-file in PEM format. |
| **MAX_MESSAGE_SIZE_MB**| Maximum size of a single message the server will accept from a client. |
| **PING_TIMEOUT**| Maximum time the server will wait for a ping from the client. |
| **PORT**| Defines which port the server will listen on. |
| **SEND_TIMEOUT**| Maximum time for the client to receive a response from the server. |
| **THREAD_LIMIT**| Defines the maximum number of threads that will handle requests.|

### 📚 Examples

```cpp
#include <iostream>

#include "json.h"
#include "lib.hpp"

int main() {
  WebTransportOpts opts(
      /* block_size_kb */ 64,
      /* cert_pem */ "certs/cert.pem",
      /* compression */ true,
      /* handshake_timeout */ 30,
      /* key_pem */ "certs/key.pem",
      /* max_message_size_kb */ 60,
      /* ping_timeout */ 30,
      /* port */ 4433,
      /* send_timeout */ 30,
      /* thread_limit */ 4);

  WebTransport wt(opts);
  WebTransportLogger wt_logger = [](const std::string& level,
                                    const std::string& message) -> void {
    std::cout << "[Arnelify Server]: " << message << std::endl;
  };

  wt.logger(wt_logger);

  WebTransportHandler wt_handler = [](WebTransportCtx& ctx,
                                      WebTransportBytes& bytes,
                                      WebTransportStream& stream) -> void {
    const WebTransportRes res = ctx;
    stream.push_json(res);
    stream.close();
  };

  wt.on("connect", wt_handler);
  wt.start();
}
```
## 🎉 TCP2 / HTTP 3.0

### 📚 Configuration

| **Option** | **Description** |
| - | - |
| **ALLOW_EMPTY_FILES**| If this option is enabled, the server will not reject empty files. |
| **BLOCK_SIZE_KB**| The size of the allocated memory used for processing large packets. |
| **CERT_PEM**| Path to the TLS cert-file in PEM format. |
| **CHARSET**| Defines the encoding that the server will recommend to all client applications. |
| **COMPRESSION**| If this option is enabled, the server will use BROTLI compression if the client application supports it. This setting increases CPU resource consumption. The server will not use compression if the data size exceeds the value of **BLOCK_SIZE_KB**. |
| **KEEP_EXTENSIONS**| If this option is enabled, file extensions will be preserved. |
| **KEY_PEM**| Path to the TLS private key-file in PEM format. |
| **MAX_FIELDS**| Defines the maximum number of fields in the received form. |
| **MAX_FIELDS_SIZE_TOTAL_MB**| Defines the maximum total size of all fields in the form. This option does not include file sizes. |
| **MAX_FILES**| Defines the maximum number of files in the form. |
| **MAX_FILES_SIZE_TOTAL_MB** | Defines the maximum total size of all files in the form. |
| **MAX_FILE_SIZE_MB**| Defines the maximum size of a single file in the form. |
| **PORT**| Defines which port the server will listen on. |
| **STORAGE_PATH**| Specifies the upload directory for storage. |
| **THREAD_LIMIT**| Defines the maximum number of threads that will handle requests. |

### 📚 Examples

```cpp
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
      /* max_fields_size_total_mb */ 1,
      /* max_files */ 3,
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
```

## 🎉 TCP1 / WebSocket

### 📚 Configuration

| **Option** | **Description** |
| - | - |
| **BLOCK_SIZE_KB**| The size of the allocated memory used for processing large packets. |
| **COMPRESSION**| If this option is enabled, the server will use BROTLI compression if the client application supports it. This setting increases CPU resource consumption. The server will not use compression if the data size exceeds the value of **BLOCK_SIZE_KB**. |
| **HANDSHAKE_TIMEOUT**| Maximum time in seconds to complete the TLS handshake. |
| **MAX_MESSAGE_SIZE_MB**| Maximum size of a single message the server will accept from a client. |
| **PING_TIMEOUT**| Maximum time the server will wait for a ping from the client. |
| **PORT**| Defines which port the server will listen on. |
| **SEND_TIMEOUT**| Maximum time for the client to receive a response from the server. |
| **THREAD_LIMIT**| Defines the maximum number of threads that will handle requests.|

### 📚 Examples

```cpp
#include <iostream>

#include "json.h"
#include "lib.hpp"

int main() {
  WebSocketOpts opts(
      /* block_size_kb */ 64,
      /* compression */ true,
      /* handshake_timeout */ 30,
      /* max_message_size_kb */ 64,
      /* ping_timeout */ 30,
      /* port */ 4433,
      /* send_timeout */ 30,
      /* thread_limit */ 4);

  WebSocket ws(opts);
  WebSocketLogger ws_logger = [](const std::string& level,
                                 const std::string& message) -> void {
    std::cout << "[Arnelify Server]: " << message << std::endl;
  };

  ws.logger(ws_logger);

  WebSocketHandler ws_handler = [](WebSocketCtx& ctx, WebSocketBytes& bytes,
                                   WebSocketStream& stream) -> void {
    const WebSocketRes res = ctx;
    stream.push_json(res);
    stream.close();
  };

  ws.on("connect", ws_handler);
  ws.start();
}
```

## 🎉 TCP1 / HTTP 2.0

### 📚 Configuration

| **Option** | **Description** |
| - | - |
| **ALLOW_EMPTY_FILES**| If this option is enabled, the server will not reject empty files. |
| **BLOCK_SIZE_KB**| The size of the allocated memory used for processing large packets. |
| **CERT_PEM**| Path to the TLS cert-file in PEM format. |
| **CHARSET**| Defines the encoding that the server will recommend to all client applications. |
| **COMPRESSION**| If this option is enabled, the server will use BROTLI compression if the client application supports it. This setting increases CPU resource consumption. The server will not use compression if the data size exceeds the value of **BLOCK_SIZE_KB**. |
| **KEEP_EXTENSIONS**| If this option is enabled, file extensions will be preserved. |
| **KEY_PEM**| Path to the TLS private key-file in PEM format. |
| **MAX_FIELDS**| Defines the maximum number of fields in the received form. |
| **MAX_FIELDS_SIZE_TOTAL_MB**| Defines the maximum total size of all fields in the form. This option does not include file sizes. |
| **MAX_FILES**| Defines the maximum number of files in the form. |
| **MAX_FILES_SIZE_TOTAL_MB** | Defines the maximum total size of all files in the form. |
| **MAX_FILE_SIZE_MB**| Defines the maximum size of a single file in the form. |
| **PORT**| Defines which port the server will listen on. |
| **STORAGE_PATH**| Specifies the upload directory for storage. |
| **THREAD_LIMIT**| Defines the maximum number of threads that will handle requests. |

### 📚 Examples

```cpp
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
      /* max_files */ 3,
      /* max_files_size_total_mb */ 60,
      /* max_file_size_mb */ 60,
      /* port */ 4433,
      /* storage_path */ "/var/www/cpp/storage",
      /* thread_limit */ 4);

  Http2 http2(opts);
  Http2Logger http2_logger = [](const std::string& level,
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
    stream.push_bytes(bytes);
    stream.end();
  };

  http2.on("/", http2_handler);
  http2.start();
}
```

## 🎉 TCP1 / HTTP 1.1

### 📚 Configuration

| **Option** | **Description** |
| - | - |
| **ALLOW_EMPTY_FILES**| If this option is enabled, the server will not reject empty files. |
| **BLOCK_SIZE_KB**| The size of the allocated memory used for processing large packets. |
| **CHARSET**| Defines the encoding that the server will recommend to all client applications. |
| **COMPRESSION**| If this option is enabled, the server will use BROTLI compression if the client application supports it. This setting increases CPU resource consumption. The server will not use compression if the data size exceeds the value of **BLOCK_SIZE_KB**. |
| **KEEP_EXTENSIONS**| If this option is enabled, file extensions will be preserved. |
| **MAX_FIELDS**| Defines the maximum number of fields in the received form. |
| **MAX_FIELDS_SIZE_TOTAL_MB**| Defines the maximum total size of all fields in the form. This option does not include file sizes. |
| **MAX_FILES**| Defines the maximum number of files in the form. |
| **MAX_FILES_SIZE_TOTAL_MB** | Defines the maximum total size of all files in the form. |
| **MAX_FILE_SIZE_MB**| Defines the maximum size of a single file in the form. |
| **PORT**| Defines which port the server will listen on. |
| **STORAGE_PATH**| Specifies the upload directory for storage. |
| **THREAD_LIMIT**| Defines the maximum number of threads that will handle requests. |

### 📚 Examples

```cpp
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
      /* max_files */ 3,
      /* max_files_size_total_mb */ 60,
      /* max_file_size_mb */ 60,
      /* port */ 4433,
      /* storage_path */ "/var/www/cpp/storage",
      /* thread_limit */ 4);

  Http1 http1(opts);
  Http1Logger http1_logger = [](const std::string& level,
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
    stream.push_bytes(bytes);
    stream.end();
  };

  http1.on("/", http1_handler);
  http1.start();
}
```

## ⚖️ MIT License
This software is licensed under the <a href="https://github.com/arnelify/arnelify-server-cpp/blob/main/LICENSE">MIT License</a>. The original author's name, logo, and the original name of the software must be included in all copies or substantial portions of the software.

## 🛠️ Contributing
Join us to help improve this software, fix bugs or implement new functionality. Active participation will help keep the software up-to-date, reliable, and aligned with the needs of its users.

Run in terminal:
```bash
docker compose up -d --build
docker ps
docker exec -it <CONTAINER ID> bash
make build
```
For TCP2 / WebTransport:
```bash
make test_wt
```
For TCP2 / HTTP 3.0:
```bash
make test_http3
```
For TCP1 / WebSocket:
```bash
make test_ws
```
For TCP1 / HTTP 2.0:
```bash
make test_http2
```
For TCP1 / HTTP 1.1:
```bash
make test_http1
```

## ⭐ Release Notes
Version 0.9.3 — a multi-language server with HTTP 3.0 and WebTransport support.

We are excited to introduce the Arnelify Server for NodeJS! Please note that this version is raw and still in active development.

Change log:

* Async Multi-Threading.
* Block processing in "on-the-fly" mode.
* BROTLI compression (still in development).
* FFI, PYO3 and NEON support.
* Significant refactoring and optimizations.

Please use this version with caution, as it may contain bugs and unfinished features. We are actively working on improving and expanding the server's capabilities, and we welcome your feedback and suggestions.

## 🔗 Links

* <a href="https://github.com/arnelify/arnelify-pod-cpp">Arnelify POD for C++</a>
* <a href="https://github.com/arnelify/arnelify-pod-node">Arnelify POD for NodeJS</a>
* <a href="https://github.com/arnelify/arnelify-pod-python">Arnelify POD for Python</a>
* <a href="https://github.com/arnelify/arnelify-pod-rust">Arnelify POD for Rust</a>
* <a href="https://github.com/arnelify/arnelify-react-native">Arnelify React Native</a>