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

//! <img src="https://static.wikia.nocookie.net/arnelify/images/c/c8/Arnelify-logo-2024.png/revision/latest?cb=20240701012515" style="width:336px;" alt="Arnelify Logo" />
//!
//! ![Arnelify Server for Rust](https://img.shields.io/badge/Arnelify%20Server%20for%20Rust-0.9.6-yellow)
//! ![Rust](https://img.shields.io/badge/Rust-1.91.1-orange)
//! ![Cargo](https://img.shields.io/badge/Cargo-1.91.1-blue)
//!
//! # About
//! **Arnelify® Server for Rust** — a multi-language server with HTTP 3.0 and WebTransport support.
//!
//! All supported protocols:
//! | **#** | **Protocol** | **Transport** |
//! | - | - | - |
//! | 1 | TCP1 | HTTP 1.1 |
//! | 2 | TCP1 | HTTP 2.0 |
//! | 3 | TCP1 | WebSocket |
//! | 4 | TCP2 | HTTP 3.0 |
//! | 5 | TCP2 | WebTransport |
//!
//! ## Minimal Requirements
//! Important: It's strongly recommended to use in a container that has been built from the gcc v15.2.0 image.
//! * CPU: Apple M1 / Intel Core i7 / AMD Ryzen 7
//! * OS: Debian 11 / MacOS 15 / Windows 10 with <a href="https://learn.microsoft.com/en-us/windows/wsl/install">WSL2</a>.
//! * RAM: 4 GB
//!
//! ## Installation
//! Run in terminal:
//! ```bash
//! cargo add arnelify_server
//! ```
//!
//! # TCP2 / WebTransport
//! **WebTransport** - is a modern protocol for low-latency, bidirectional data transfer in browsers and applications. It works over Next Generation HTTP 3.0 and Next Generation TCP2 (QUIC), allowing multiple streams and messages in parallel.
//!
//! ## Configuration
//!
//! | **Option** | **Description** |
//! | - | - |
//! | **BLOCK_SIZE_KB**| The size of the allocated memory used for processing large packets. |
//! | **CERT_PEM**| Path to the TLS cert-file in PEM format. |
//! | **COMPRESSION**| If this option is enabled, the server will use BROTLI compression if the client application supports it. This setting increases CPU resource consumption. The server will not use compression if the data size exceeds the value of **BLOCK_SIZE_KB**. |
//! | **HANDSHAKE_TIMEOUT**| Maximum time in seconds to complete the TLS handshake. |
//! | **KEY_PEM**| Path to the TLS private key-file in PEM format. |
//! | **MAX_MESSAGE_SIZE_MB**| Maximum size of a single message the server will accept from a client. |
//! | **PING_TIMEOUT**| Maximum time the server will wait for a ping from the client. |
//! | **PORT**| Defines which port the server will listen on. |
//! | **SEND_TIMEOUT**| Maximum time for the client to receive a response from the server. |
//! | **THREAD_LIMIT**| Defines the maximum number of threads that will handle requests.|
//!
//! ## Examples
//!
//! ```rust
//! use arnelify_server::tcp2::WebTransport;
//! use arnelify_server::tcp2::WebTransportBytes;
//! use arnelify_server::tcp2::WebTransportCtx;
//! use arnelify_server::tcp2::WebTransportHandler;
//! use arnelify_server::tcp2::WebTransportLogger;
//! use arnelify_server::tcp2::WebTransportOpts;
//! use arnelify_server::tcp2::WebTransportStream;
//! use std::sync::{Arc, Mutex};
//!
//! type JSON = serde_json::Value;
//!
//! fn main() {
//!   let wt_opts: WebTransportOpts = WebTransportOpts {
//!     block_size_kb: 64,
//!     cert_pem: String::from("certs/cert.pem"),
//!     compression: false,
//!     handshake_timeout: 30,
//!     key_pem: String::from("certs/key.pem"),
//!     max_message_size_kb: 64,
//!     ping_timeout: 15,
//!     port: 4433,
//!     send_timeout: 30,
//!     thread_limit: 4,
//!   };
//!
//!   let wt: WebTransport = WebTransport::new(wt_opts);
//!   let wt_logger: Arc<WebTransportLogger> = Arc::new(move |_level: &str, message: &str| {
//!     println!("[Arnelify Server]: {}", message);
//!   });
//!
//!   wt.logger(wt_logger);
//!   let wt_handler: Arc<WebTransportHandler> = Arc::new(
//!     move |ctx: Arc<Mutex<WebTransportCtx>>,
//!           bytes: Arc<Mutex<WebTransportBytes>>,
//!           stream: Arc<Mutex<WebTransportStream>>| {
//!       let json: JSON = ctx.lock().unwrap().clone();
//!       let bytes: Vec<u8> = bytes.lock().unwrap().clone();
//!       let mut stream_lock: std::sync::MutexGuard<'_, WebTransportStream> = stream.lock().unwrap();
//!       stream_lock.push(&json, &bytes);
//!       stream_lock.close();
//!     },
//!   );
//!
//!   wt.on("connect", wt_handler);
//!   wt.start();
//! }
//! ```
//!
//! # TCP2 / HTTP 3.0
//! **HTTP 3.0** - is the latest version of the HTTP protocol, built on top of Next Generation TCP2 (QUIC) instead of TCP1. It provides faster and more reliable connections by reducing handshake latency and improving packet loss recovery.
//!
//! ## Configuration
//!
//! | **Option** | **Description** |
//! | - | - |
//! | **ALLOW_EMPTY_FILES**| If this option is enabled, the server will not reject empty files. |
//! | **BLOCK_SIZE_KB**| The size of the allocated memory used for processing large packets. |
//! | **CERT_PEM**| Path to the TLS cert-file in PEM format. |
//! | **CHARSET**| Defines the encoding that the server will recommend to all client applications. |
//! | **COMPRESSION**| If this option is enabled, the server will use BROTLI compression if the client application supports it. This setting increases CPU resource consumption. The server will not use compression if the data size exceeds the value of **BLOCK_SIZE_KB**. |
//! | **KEEP_EXTENSIONS**| If this option is enabled, file extensions will be preserved. |
//! | **KEY_PEM**| Path to the TLS private key-file in PEM format. |
//! | **MAX_FIELDS**| Defines the maximum number of fields in the received form. |
//! | **MAX_FIELDS_SIZE_TOTAL_MB**| Defines the maximum total size of all fields in the form. This option does not include file sizes. |
//! | **MAX_FILES**| Defines the maximum number of files in the form. |
//! | **MAX_FILES_SIZE_TOTAL_MB** | Defines the maximum total size of all files in the form. |
//! | **MAX_FILE_SIZE_MB**| Defines the maximum size of a single file in the form. |
//! | **PORT**| Defines which port the server will listen on. |
//! | **STORAGE_PATH**| Specifies the upload directory for storage. |
//! | **THREAD_LIMIT**| Defines the maximum number of threads that will handle requests. |
//!
//! ## Examples
//!
//! ```rust
//! use arnelify_server::tcp2::Http3;
//! use arnelify_server::tcp2::Http3Ctx;
//! use arnelify_server::tcp2::Http3Handler;
//! use arnelify_server::tcp2::Http3Logger;
//! use arnelify_server::tcp2::Http3Opts;
//! use arnelify_server::tcp2::Http3Stream;
//! use std::sync::{Arc, Mutex};
//!
//! type JSON = serde_json::Value;
//!
//! fn main() {
//!   let http3_opts: Http3Opts = Http3Opts {
//!     allow_empty_files: true,
//!     block_size_kb: 64,
//!     cert_pem: String::from("certs/cert.pem"),
//!     charset: String::from("utf-8"),
//!     compression: true,
//!     keep_alive: 30,
//!     keep_extensions: true,
//!     key_pem: String::from("certs/key.pem"),
//!     max_fields: 60,
//!     max_fields_size_total_mb: 1,
//!     max_files: 1,
//!     max_files_size_total_mb: 60,
//!     max_file_size_mb: 60,
//!     port: 4433,
//!     storage_path: String::from("/var/www/rust/storage"),
//!     thread_limit: 4,
//!   };
//!
//!   let http3: Http3 = Http3::new(http3_opts);
//!   let http3_logger: Arc<Http3Logger> = Arc::new(move |_level: &str, message: &str| {
//!     println!("[Arnelify Server]: {}", message);
//!   });
//!
//!   http3.logger(http3_logger);
//!   let http3_handler: Arc<Http3Handler> = Arc::new(
//!     move |ctx: Arc<Mutex<Http3Ctx>>, stream: Arc<Mutex<Http3Stream>>| {
//!       let json: JSON = ctx.lock().unwrap().clone();
//!       let mut stream_lock: std::sync::MutexGuard<'_, Http3Stream> = stream.lock().unwrap();
//!       stream_lock.set_code(200);
//!       stream_lock.push_json(&json, false);
//!       stream_lock.end();
//!     },
//!   );
//!
//!   http3.on("/", http3_handler);
//!   http3.start();
//! }
//! ```
//!
//! # TCP1 / WebSocket
//! **WebSocket** - is a protocol that enables full-duplex, bidirectional communication between a client and server over a single TCP1 connection.
//!
//! ## Configuration
//!
//! | **Option** | **Description** |
//! | - | - |
//! | **BLOCK_SIZE_KB**| The size of the allocated memory used for processing large packets. |
//! | **COMPRESSION**| If this option is enabled, the server will use BROTLI compression if the client application supports it. This setting increases CPU resource consumption. The server will not use compression if the data size exceeds the value of **BLOCK_SIZE_KB**. |
//! | **HANDSHAKE_TIMEOUT**| Maximum time in seconds to complete the TLS handshake. |
//! | **MAX_MESSAGE_SIZE_MB**| Maximum size of a single message the server will accept from a client. |
//! | **PING_TIMEOUT**| Maximum time the server will wait for a ping from the client. |
//! | **PORT**| Defines which port the server will listen on. |
//! | **SEND_TIMEOUT**| Maximum time for the client to receive a response from the server. |
//! | **THREAD_LIMIT**| Defines the maximum number of threads that will handle requests.|
//!
//! ## Examples
//!
//! ```rust
//! use arnelify_server::tcp1::WebSocket;
//! use arnelify_server::tcp1::WebSocketBytes;
//! use arnelify_server::tcp1::WebSocketCtx;
//! use arnelify_server::tcp1::WebSocketHandler;
//! use arnelify_server::tcp1::WebSocketLogger;
//! use arnelify_server::tcp1::WebSocketOpts;
//! use arnelify_server::tcp1::WebSocketStream;
//! use std::sync::{Arc, Mutex};
//!
//! type JSON = serde_json::Value;
//!
//! fn main() {
//!   let ws_opts: WebSocketOpts = WebSocketOpts {
//!     block_size_kb: 64,
//!     compression: false,
//!     handshake_timeout: 30,
//!     max_message_size_kb: 64,
//!     ping_timeout: 15,
//!     port: 4433,
//!     send_timeout: 30,
//!     thread_limit: 4,
//!   };
//!
//!   let ws: WebSocket = WebSocket::new(ws_opts);
//!   let ws_logger: Arc<WebSocketLogger> = Arc::new(move |_level: &str, message: &str| {
//!     println!("[Arnelify Server]: {}", message);
//!   });
//!
//!   ws.logger(ws_logger);
//!   let ws_handler: Arc<WebSocketHandler> = Arc::new(
//!     move |ctx: Arc<Mutex<WebSocketCtx>>,
//!           bytes: Arc<Mutex<WebSocketBytes>>,
//!           stream: Arc<Mutex<WebSocketStream>>| {
//!       let json: JSON = ctx.lock().unwrap().clone();
//!       let bytes: Vec<u8> = bytes.lock().unwrap().clone();
//!       let mut stream_lock: std::sync::MutexGuard<'_, WebSocketStream> = stream.lock().unwrap();
//!       stream_lock.push(&json, &bytes);
//!       stream_lock.close();
//!     },
//!   );
//!
//!   ws.on("connect", ws_handler);
//!   ws.start();
//! }
//! ```
//!
//! # TCP1 / HTTP 2.0
//! **HTTP 2.0** - is a revision of the HTTP protocol that improves performance over HTTP 1.1. It introduces multiplexed streams, allowing multiple requests and responses to be sent over a single TCP1 connection simultaneously.
//!
//! ## Configuration
//!
//! | **Option** | **Description** |
//! | - | - |
//! | **ALLOW_EMPTY_FILES**| If this option is enabled, the server will not reject empty files. |
//! | **BLOCK_SIZE_KB**| The size of the allocated memory used for processing large packets. |
//! | **CERT_PEM**| Path to the TLS cert-file in PEM format. |
//! | **CHARSET**| Defines the encoding that the server will recommend to all client applications. |
//! | **COMPRESSION**| If this option is enabled, the server will use BROTLI compression if the client application supports it. This setting increases CPU resource consumption. The server will not use compression if the data size exceeds the value of **BLOCK_SIZE_KB**. |
//! | **KEEP_EXTENSIONS**| If this option is enabled, file extensions will be preserved. |
//! | **KEY_PEM**| Path to the TLS private key-file in PEM format. |
//! | **MAX_FIELDS**| Defines the maximum number of fields in the received form. |
//! | **MAX_FIELDS_SIZE_TOTAL_MB**| Defines the maximum total size of all fields in the form. This option does not include file sizes. |
//! | **MAX_FILES**| Defines the maximum number of files in the form. |
//! | **MAX_FILES_SIZE_TOTAL_MB** | Defines the maximum total size of all files in the form. |
//! | **MAX_FILE_SIZE_MB**| Defines the maximum size of a single file in the form. |
//! | **PORT**| Defines which port the server will listen on. |
//! | **STORAGE_PATH**| Specifies the upload directory for storage. |
//! | **THREAD_LIMIT**| Defines the maximum number of threads that will handle requests. |
//!
//! ## Examples
//!
//! ```rust
//! use arnelify_server::tcp1::Http2;
//! use arnelify_server::tcp1::Http2Ctx;
//! use arnelify_server::tcp1::Http2Handler;
//! use arnelify_server::tcp1::Http2Logger;
//! use arnelify_server::tcp1::Http2Opts;
//! use arnelify_server::tcp1::Http2Stream;
//! use std::sync::{Arc, Mutex};
//!
//! type JSON = serde_json::Value;
//!
//! fn main() {
//!   let http2_opts: Http2Opts = Http2Opts {
//!     allow_empty_files: true,
//!     block_size_kb: 64,
//!     cert_pem: String::from("certs/cert.pem"),
//!     charset: String::from("utf-8"),
//!     compression: true,
//!     keep_alive: 30,
//!     keep_extensions: true,
//!     key_pem: String::from("certs/key.pem"),
//!     max_fields: 60,
//!     max_fields_size_total_mb: 1,
//!     max_files: 1,
//!     max_files_size_total_mb: 60,
//!     max_file_size_mb: 60,
//!     port: 4433,
//!     storage_path: String::from("/var/www/rust/storage"),
//!     thread_limit: 4,
//!   };
//!
//!   let http2: Http2 = Http2::new(http2_opts);
//!   let http2_logger: Arc<Http2Logger> = Arc::new(move |_level: &str, message: &str| {
//!     println!("[Arnelify Server]: {}", message);
//!   });
//!
//!   http2.logger(http2_logger);
//!   let http2_handler: Arc<Http2Handler> = Arc::new(
//!     move |ctx: Arc<Mutex<Http2Ctx>>, stream: Arc<Mutex<Http2Stream>>| {
//!       let json: JSON = ctx.lock().unwrap().clone();
//!       let mut stream_lock: std::sync::MutexGuard<'_, Http2Stream> = stream.lock().unwrap();
//!       stream_lock.set_code(200);
//!       stream_lock.push_json(&json, false);
//!       stream_lock.end();
//!     },
//!   );
//!
//!   http2.on("/", http2_handler);
//!   http2.start();
//! }
//! ```
//!
//! # TCP1 / HTTP 1.1
//! **HTTP 1.1** - is a widely used version of the HTTP protocol that relies on TCP1 connections for communication.
//!
//! ## Configuration
//!
//! | **Option** | **Description** |
//! | - | - |
//! | **ALLOW_EMPTY_FILES**| If this option is enabled, the server will not reject empty files. |
//! | **BLOCK_SIZE_KB**| The size of the allocated memory used for processing large packets. |
//! | **CHARSET**| Defines the encoding that the server will recommend to all client applications. |
//! | **COMPRESSION**| If this option is enabled, the server will use BROTLI compression if the client application supports it. This setting increases CPU resource consumption. The server will not use compression if the data size exceeds the value of **BLOCK_SIZE_KB**. |
//! | **KEEP_EXTENSIONS**| If this option is enabled, file extensions will be preserved. |
//! | **MAX_FIELDS**| Defines the maximum number of fields in the received form. |
//! | **MAX_FIELDS_SIZE_TOTAL_MB**| Defines the maximum total size of all fields in the form. This option does not include file sizes. |
//! | **MAX_FILES**| Defines the maximum number of files in the form. |
//! | **MAX_FILES_SIZE_TOTAL_MB** | Defines the maximum total size of all files in the form. |
//! | **MAX_FILE_SIZE_MB**| Defines the maximum size of a single file in the form. |
//! | **PORT**| Defines which port the server will listen on. |
//! | **STORAGE_PATH**| Specifies the upload directory for storage. |
//! | **THREAD_LIMIT**| Defines the maximum number of threads that will handle requests. |
//!
//! ## Examples
//!
//! ```rust
//! use arnelify_server::tcp1::Http1;
//! use arnelify_server::tcp1::Http1Ctx;
//! use arnelify_server::tcp1::Http1Handler;
//! use arnelify_server::tcp1::Http1Logger;
//! use arnelify_server::tcp1::Http1Opts;
//! use arnelify_server::tcp1::Http1Stream;
//! use std::sync::{Arc, Mutex};
//!
//! type JSON = serde_json::Value;
//!
//! fn main() {
//!   let http1_opts: Http1Opts = Http1Opts {
//!     allow_empty_files: true,
//!     block_size_kb: 64,
//!     charset: String::from("utf-8"),
//!     compression: true,
//!     keep_alive: 30,
//!     keep_extensions: true,
//!     max_fields: 60,
//!     max_fields_size_total_mb: 1,
//!     max_files: 1,
//!     max_files_size_total_mb: 60,
//!     max_file_size_mb: 60,
//!     port: 4433,
//!     storage_path: String::from("/var/www/rust/storage"),
//!     thread_limit: 4,
//!   };
//!
//!   let http1: Http1 = Http1::new(http1_opts);
//!   let http1_logger: Arc<Http1Logger> = Arc::new(move |_level: &str, message: &str| {
//!     println!("[Arnelify Server]: {}", message);
//!   });
//!
//!   http1.logger(http1_logger);
//!   let http1_handler: Arc<Http1Handler> = Arc::new(
//!     move |ctx: Arc<Mutex<Http1Ctx>>, stream: Arc<Mutex<Http1Stream>>| {
//!       let json: JSON = ctx.lock().unwrap().clone();
//!       let mut stream_lock: std::sync::MutexGuard<'_, Http1Stream> = stream.lock().unwrap();
//!       stream_lock.set_code(200);
//!       stream_lock.push_json(&json, false);
//!       stream_lock.end();
//!     },
//!   );
//!
//!   http1.on("/", http1_handler);
//!   http1.start();
//! }
//! ```
//!
//! # MIT License
//!
//! This software is licensed under the <a href="https://github.com/arnelify/arnelify-server-rust/blob/main/LICENSE">MIT License</a>. The original author's name, logo, and the original name of the software must be included in all copies or substantial portions of the software.
//!
//! # Contributing
//!
//! Join us to help improve this software, fix bugs or implement new functionality. Active participation will help keep the software up-to-date, reliable, and aligned with the needs of its users.
//!
//! Run in terminal:
//! ```bash
//! docker compose up -d --build
//! docker ps
//! docker exec -it <CONTAINER ID> bash
//! ```
//! For TCP2 / WebTransport:
//! ```bash
//! cargo run --bin test_wt
//! ```
//! For TCP2 / HTTP 3.0:
//! ```bash
//! cargo run --bin test_http3
//! ```
//! For TCP1 / WebSocket:
//! ```bash
//! cargo run --bin test_ws
//! ```
//! For TCP1 / HTTP 2.0:
//! ```bash
//! cargo run --bin test_http2
//! ```
//! For TCP1 / HTTP 1.1:
//! ```bash
//! cargo run --bin test_http1
//! ```
//! # Release Notes
//!
//! Version 0.9.6 — a multi-language server with HTTP 3.0 and WebTransport support.
//!
//! We are excited to introduce the Arnelify Server for Rust! Please note that this version is raw and still in active development.
//!
//! Change Log:
//!
//! * Async Multi-Threading.
//! * Block processing in "on-the-fly" mode.
//! * BROTLI compression (still in development).
//! * FFI, PYO3 and NEON support.
//! * Significant refactoring and optimizations.
//!
//! Please use this version with caution, as it may contain bugs and unfinished features. We are actively working on improving and expanding the server's capabilities, and we welcome your feedback and suggestions.
//!
//! # Links
//!
//! * <a href="https://github.com/arnelify/arnelify-pod-cpp">Arnelify POD for C++</a>
//! * <a href="https://github.com/arnelify/arnelify-pod-node">Arnelify POD for NodeJS</a>
//! * <a href="https://github.com/arnelify/arnelify-pod-python">Arnelify POD for Python</a>
//! * <a href="https://github.com/arnelify/arnelify-pod-rust">Arnelify POD for Rust</a>
//! * <a href="https://github.com/arnelify/arnelify-react-native">Arnelify React Native</a>

pub mod tcp1;
pub mod tcp2;

pub use tcp1::{Http1, Http1Ctx, Http1Handler, Http1Logger, Http1Opts, Http1Stream};
pub use tcp1::{Http2, Http2Ctx, Http2Handler, Http2Logger, Http2Opts, Http2Stream};
pub use tcp1::{
  WebSocket, WebSocketBytes, WebSocketCtx, WebSocketHandler, WebSocketLogger, WebSocketOpts,
  WebSocketStream,
};

pub use tcp2::{Http3, Http3Ctx, Http3Handler, Http3Logger, Http3Opts, Http3Stream};
pub use tcp2::{
  WebTransport, WebTransportBytes, WebTransportCtx, WebTransportHandler, WebTransportLogger,
  WebTransportOpts, WebTransportStream,
};

use std::{
  collections::HashMap,
  convert::TryFrom,
  ffi::{CStr, CString},
  os::raw::{c_char, c_int},
  slice,
  sync::{
    Arc, Mutex, MutexGuard, OnceLock,
    atomic::{AtomicI32, Ordering},
  },
};

use serde_json::Value as JSON;

type Http1Streams = HashMap<i32, Arc<Mutex<Http1Stream>>>;
type WebSocketStreams = HashMap<i32, Arc<Mutex<WebSocketStream>>>;
type Http2Streams = HashMap<i32, Arc<Mutex<Http2Stream>>>;
type Http3Streams = HashMap<i32, Arc<Mutex<Http3Stream>>>;
type WebTransportStreams = HashMap<i32, Arc<Mutex<WebTransportStream>>>;

type Handler = extern "C" fn(c_id: c_int, c_stream_id: c_int, c_json: *const c_char);
type HandlerWithTransport = extern "C" fn(
  c_id: c_int,
  c_stream_id: c_int,
  c_json: *const c_char,
  c_bytes: *const c_char,
  c_bytes_len: c_int,
);

type Logger = extern "C" fn(c_id: c_int, c_level: *const c_char, c_message: *const c_char);

static HTTP1_MAP: OnceLock<Mutex<HashMap<c_int, Arc<Http1>>>> = OnceLock::new();
static HTTP1_ID: OnceLock<Mutex<c_int>> = OnceLock::new();
static HTTP1_STREAM_ID: AtomicI32 = AtomicI32::new(1);
static HTTP1_STREAMS: OnceLock<Mutex<Http1Streams>> = OnceLock::new();

static WS_MAP: OnceLock<Mutex<HashMap<c_int, Arc<WebSocket>>>> = OnceLock::new();
static WS_ID: OnceLock<Mutex<c_int>> = OnceLock::new();
static WS_STREAM_ID: AtomicI32 = AtomicI32::new(1);
static WS_STREAMS: OnceLock<Mutex<WebSocketStreams>> = OnceLock::new();

static HTTP2_MAP: OnceLock<Mutex<HashMap<c_int, Arc<Http2>>>> = OnceLock::new();
static HTTP2_ID: OnceLock<Mutex<c_int>> = OnceLock::new();
static HTTP2_STREAM_ID: AtomicI32 = AtomicI32::new(1);
static HTTP2_STREAMS: OnceLock<Mutex<Http2Streams>> = OnceLock::new();

static HTTP3_MAP: OnceLock<Mutex<HashMap<c_int, Arc<Http3>>>> = OnceLock::new();
static HTTP3_ID: OnceLock<Mutex<c_int>> = OnceLock::new();
static HTTP3_STREAM_ID: AtomicI32 = AtomicI32::new(1);
static HTTP3_STREAMS: OnceLock<Mutex<Http3Streams>> = OnceLock::new();

static WT_MAP: OnceLock<Mutex<HashMap<c_int, Arc<WebTransport>>>> = OnceLock::new();
static WT_ID: OnceLock<Mutex<c_int>> = OnceLock::new();
static WT_STREAM_ID: AtomicI32 = AtomicI32::new(1);
static WT_STREAMS: OnceLock<Mutex<WebTransportStreams>> = OnceLock::new();

fn get_str(opts: &JSON, key: &str) -> String {
  opts
    .get(key)
    .and_then(JSON::as_str)
    .expect(&format!(
      "[Arnelify Server]: Rust FFI error: '{}' missing or not a string.",
      key
    ))
    .to_string()
}

fn get_u64(opts: &JSON, key: &str) -> u64 {
  opts.get(key).and_then(JSON::as_u64).expect(&format!(
    "[Arnelify Server]: Rust FFI error: '{}' missing or not a u64.",
    key
  ))
}

fn get_usize(opts: &JSON, key: &str) -> usize {
  let val: u64 = get_u64(opts, key);
  usize::try_from(val).expect(&format!(
    "[Arnelify Server]: Rust FFI error: '{}' out of usize range.",
    key
  ))
}

fn get_u32(opts: &JSON, key: &str) -> u32 {
  let val: u64 = get_u64(opts, key);
  u32::try_from(val).expect(&format!(
    "[Arnelify Server]: Rust FFI error: '{}' out of u32 range.",
    key
  ))
}

fn get_u16(opts: &JSON, key: &str) -> u16 {
  let val: u64 = get_u64(opts, key);
  u16::try_from(val).expect(&format!(
    "[Arnelify Server]: Rust FFI error: '{}' out of u16 range.",
    key
  ))
}

fn get_u8(opts: &JSON, key: &str) -> u8 {
  let val: u64 = get_u64(opts, key);
  u8::try_from(val).expect(&format!(
    "[Arnelify Server]: Rust FFI error: '{}' out of u8 range.",
    key
  ))
}

fn get_bool(opts: &JSON, key: &str) -> bool {
  opts.get(key).and_then(JSON::as_bool).expect(&format!(
    "[Arnelify Server]: Rust FFI error: '{}' missing or not a bool.",
    key
  ))
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_add_header(
  c_stream_id: c_int,
  c_key: *const c_char,
  c_value: *const c_char,
) {
  if let Some(map) = HTTP1_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http1Stream>>> = {
      let streams: MutexGuard<'_, Http1Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let key: &str = match unsafe { CStr::from_ptr(c_key) }.to_str() {
          Ok(s) => s,
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http1_add_header: Invalid UTF-8 in 'c_key'."
            );
            return;
          }
        };

        let value: &str = match unsafe { CStr::from_ptr(c_value) }.to_str() {
          Ok(s) => s,
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http1_add_header: Invalid UTF-8 in 'c_value'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, Http1Stream> = stream.lock().unwrap();
        stream_lock.add_header(key, value);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http1_add_header: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_create(c_opts: *const c_char) -> c_int {
  let opts: JSON = match unsafe { CStr::from_ptr(c_opts) }.to_str() {
    Ok(s) => match serde_json::from_str(s) {
      Ok(json) => json,
      Err(_) => {
        println!("[Arnelify Server]: Rust FFI error in http1_create: Invalid JSON in 'c_opts'.");
        return 0;
      }
    },
    Err(_) => {
      println!("[Arnelify Server]: Rust FFI error in http1_create: Invalid UTF-8 in 'c_opts'.");
      return 0;
    }
  };

  let map: &Mutex<HashMap<c_int, Arc<Http1>>> =
    HTTP1_MAP.get_or_init(|| Mutex::new(HashMap::new()));
  let id: &Mutex<c_int> = HTTP1_ID.get_or_init(|| Mutex::new(0));
  let c_id: c_int = {
    let mut c: MutexGuard<'_, c_int> = id.lock().unwrap();
    *c += 1;
    *c
  };

  let http1_opts: Http1Opts = Http1Opts {
    allow_empty_files: get_bool(&opts, "allow_empty_files"),
    block_size_kb: get_usize(&opts, "block_size_kb"),
    charset: get_str(&opts, "charset"),
    compression: get_bool(&opts, "compression"),
    keep_alive: get_u8(&opts, "keep_alive"),
    keep_extensions: get_bool(&opts, "keep_extensions"),
    max_fields: get_u32(&opts, "max_fields"),
    max_fields_size_total_mb: get_usize(&opts, "max_fields_size_total_mb"),
    max_files: get_u32(&opts, "max_files"),
    max_files_size_total_mb: get_usize(&opts, "max_files_size_total_mb"),
    max_file_size_mb: get_usize(&opts, "max_file_size_mb"),
    port: get_u16(&opts, "port"),
    storage_path: get_str(&opts, "storage_path"),
    thread_limit: get_u64(&opts, "thread_limit"),
  };

  {
    let http1: Http1 = Http1::new(http1_opts);
    map.lock().unwrap().insert(c_id, Arc::new(http1));
  }

  c_id
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_destroy(c_id: c_int) {
  if let Some(map) = HTTP1_MAP.get() {
    map.lock().unwrap().remove(&c_id);
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_end(c_stream_id: c_int) {
  if let Some(map) = HTTP1_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http1Stream>>> = {
      let streams: MutexGuard<'_, Http1Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let mut stream_lock: std::sync::MutexGuard<'_, Http1Stream> = stream.lock().unwrap();
        stream_lock.end();
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http1_end: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_logger(c_id: c_int, c_cb: Logger) {
  let http1_logger: Arc<Http1Logger> = Arc::new(move |level: &str, message: &str| {
    let c_level: CString = CString::new(level).unwrap();
    let c_message: CString = CString::new(message).unwrap();
    c_cb(c_id, c_level.as_ptr(), c_message.as_ptr());
  });

  if let Some(map) = HTTP1_MAP.get() {
    if let Some(http1) = map.lock().unwrap().get_mut(&c_id) {
      http1.logger(http1_logger);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_on(c_id: c_int, c_path: *const c_char, c_cb: Handler) {
  let path: &str = match unsafe { CStr::from_ptr(c_path) }.to_str() {
    Ok(s) => s,
    Err(_) => {
      println!("[Arnelify Server]: Rust FFI error in http1_on: Invalid UTF-8 in 'c_path'.");
      return;
    }
  };

  let http1_handler: Arc<Http1Handler> = Arc::new(
    move |ctx: Arc<Mutex<Http1Ctx>>, stream: Arc<Mutex<Http1Stream>>| {
      let stream_id: i32 = HTTP1_STREAM_ID.fetch_add(1, Ordering::Relaxed);

      HTTP1_STREAMS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
        .insert(stream_id, stream);

      let json: String = {
        let ctx_lock: MutexGuard<'_, Http1Ctx> = ctx.lock().unwrap();
        serde_json::to_string(&*ctx_lock).unwrap()
      };

      let c_json: CString = CString::new(json).unwrap();
      c_cb(c_id, stream_id, c_json.as_ptr());

      if let Some(map) = HTTP1_STREAMS.get() {
        map.lock().unwrap().remove(&stream_id);
      }
    },
  );

  if let Some(map) = HTTP1_MAP.get() {
    if let Some(http1) = map.lock().unwrap().get_mut(&c_id) {
      http1.on(path, http1_handler);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_push_bytes(
  c_stream_id: c_int,
  c_bytes: *const c_char,
  c_bytes_len: c_int,
  c_is_attachment: c_int,
) {
  if let Some(map) = HTTP1_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http1Stream>>> = {
      let streams: MutexGuard<'_, Http1Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let is_attachment: bool = c_is_attachment == 1;
        if c_bytes.is_null() || 0 >= c_bytes_len {
          let mut stream_lock: std::sync::MutexGuard<'_, Http1Stream> = stream.lock().unwrap();
          stream_lock.push_bytes(&[], is_attachment);
          return;
        }

        let bytes: &[u8] =
          unsafe { slice::from_raw_parts(c_bytes as *const u8, c_bytes_len as usize) };
        let mut stream_lock: std::sync::MutexGuard<'_, Http1Stream> = stream.lock().unwrap();
        stream_lock.push_bytes(bytes, is_attachment);
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http1_push_bytes: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_push_file(
  c_stream_id: c_int,
  c_file_path: *const c_char,
  c_is_attachment: c_int,
) {
  if let Some(map) = HTTP1_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http1Stream>>> = {
      let streams: MutexGuard<'_, Http1Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let is_attachment: bool = c_is_attachment == 1;
        let file_path: &str = match unsafe { CStr::from_ptr(c_file_path) }.to_str() {
          Ok(s) => s,
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http1_push_file: Invalid UTF-8 in 'c_file_path'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, Http1Stream> = stream.lock().unwrap();
        stream_lock.push_file(file_path, is_attachment);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http1_push_file: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_push_json(
  c_stream_id: c_int,
  c_json: *const c_char,
  c_is_attachment: c_int,
) {
  if let Some(map) = HTTP1_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http1Stream>>> = {
      let streams: MutexGuard<'_, Http1Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let is_attachment: bool = c_is_attachment == 1;
        let json: JSON = match unsafe { CStr::from_ptr(c_json) }.to_str() {
          Ok(s) => match serde_json::from_str(s) {
            Ok(json) => json,
            Err(_) => {
              println!(
                "[Arnelify Server]: Rust FFI error in http1_push_json: Invalid JSON in 'c_json'."
              );
              return;
            }
          },
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http1_push_json: Invalid UTF-8 in 'c_json'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, Http1Stream> = stream.lock().unwrap();
        stream_lock.push_json(&json, is_attachment);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http1_push_json: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_set_code(c_stream_id: c_int, c_code: c_int) {
  if let Some(map) = HTTP1_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http1Stream>>> = {
      let streams: MutexGuard<'_, Http1Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let mut stream_lock: std::sync::MutexGuard<'_, Http1Stream> = stream.lock().unwrap();
        stream_lock.set_code(c_code as u16);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http1_set_code: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_set_compression(c_stream_id: c_int, c_compression: *const c_char) {
  if let Some(map) = HTTP1_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http1Stream>>> = {
      let streams: MutexGuard<'_, Http1Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let compression: &str = match unsafe { CStr::from_ptr(c_compression) }.to_str() {
          Ok(s) => s,
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http1_set_compression: Invalid UTF-8 in 'c_compression'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, Http1Stream> = stream.lock().unwrap();
        if compression.len() > 0 {
          stream_lock.set_compression(Some(String::from(compression)));
          return;
        }

        stream_lock.set_compression(None);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http1_set_compression: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_set_headers(c_stream_id: c_int, c_headers: *const c_char) {
  if let Some(map) = HTTP1_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http1Stream>>> = {
      let streams: MutexGuard<'_, Http1Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let json: Vec<JSON> = match unsafe { CStr::from_ptr(c_headers) }.to_str() {
          Ok(s) => match serde_json::from_str(s) {
            Ok(json) => json,
            Err(_) => {
              println!(
                "[Arnelify Server]: Rust FFI error in http1_set_headers: Invalid JSON in 'c_headers'."
              );
              return;
            }
          },
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http1_set_headers: Invalid UTF-8 in 'c_headers'."
            );
            return;
          }
        };

        let mut headers: Vec<(String, String)> = Vec::new();
        for header in json {
          if let JSON::Object(pair) = header {
            for (key, value) in pair {
              let value = match value {
                JSON::String(s) => s,
                JSON::Number(n) => n.to_string(),
                JSON::Bool(b) => b.to_string(),
                _ => continue,
              };
              headers.push((key, value));
            }
          }
        }

        let mut stream_lock: std::sync::MutexGuard<'_, Http1Stream> = stream.lock().unwrap();
        stream_lock.set_headers(headers);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http1_set_headers: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_start(c_id: c_int) {
  if let Some(map) = HTTP1_MAP.get() {
    if let Some(http1) = map.lock().unwrap().get_mut(&c_id) {
      http1.start();
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http1_stop(c_id: c_int) {
  if let Some(map) = HTTP1_MAP.get() {
    if let Some(http1) = map.lock().unwrap().get_mut(&c_id) {
      http1.stop();
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_add_header(
  c_stream_id: c_int,
  c_key: *const c_char,
  c_value: *const c_char,
) {
  if let Some(map) = HTTP2_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http2Stream>>> = {
      let streams: MutexGuard<'_, Http2Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let key: &str = match unsafe { CStr::from_ptr(c_key) }.to_str() {
          Ok(s) => s,
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http2_add_header: Invalid UTF-8 in 'c_key'."
            );
            return;
          }
        };

        let value: &str = match unsafe { CStr::from_ptr(c_value) }.to_str() {
          Ok(s) => s,
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http2_add_header: Invalid UTF-8 in 'c_value'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, Http2Stream> = stream.lock().unwrap();
        stream_lock.add_header(key, value);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http2_add_header: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_create(c_opts: *const c_char) -> c_int {
  let opts: JSON = match unsafe { CStr::from_ptr(c_opts) }.to_str() {
    Ok(s) => match serde_json::from_str(s) {
      Ok(json) => json,
      Err(_) => {
        println!("[Arnelify Server]: Rust FFI error in http2_create: Invalid JSON in 'c_opts'.");
        return 0;
      }
    },
    Err(_) => {
      println!("[Arnelify Server]: Rust FFI error in http2_create: Invalid UTF-8 in 'c_opts'.");
      return 0;
    }
  };

  let map: &Mutex<HashMap<c_int, Arc<Http2>>> =
    HTTP2_MAP.get_or_init(|| Mutex::new(HashMap::new()));
  let id: &Mutex<c_int> = HTTP2_ID.get_or_init(|| Mutex::new(0));
  let c_id: c_int = {
    let mut c: MutexGuard<'_, c_int> = id.lock().unwrap();
    *c += 1;
    *c
  };

  let http2_opts: Http2Opts = Http2Opts {
    allow_empty_files: get_bool(&opts, "allow_empty_files"),
    block_size_kb: get_usize(&opts, "block_size_kb"),
    charset: get_str(&opts, "charset"),
    cert_pem: get_str(&opts, "cert_pem"),
    compression: get_bool(&opts, "compression"),
    keep_alive: get_u8(&opts, "keep_alive"),
    keep_extensions: get_bool(&opts, "keep_extensions"),
    key_pem: get_str(&opts, "key_pem"),
    max_fields: get_u32(&opts, "max_fields"),
    max_fields_size_total_mb: get_usize(&opts, "max_fields_size_total_mb"),
    max_files: get_u32(&opts, "max_files"),
    max_files_size_total_mb: get_usize(&opts, "max_files_size_total_mb"),
    max_file_size_mb: get_usize(&opts, "max_file_size_mb"),
    port: get_u16(&opts, "port"),
    storage_path: get_str(&opts, "storage_path"),
    thread_limit: get_u64(&opts, "thread_limit"),
  };

  {
    let http2: Http2 = Http2::new(http2_opts);
    map.lock().unwrap().insert(c_id, Arc::new(http2));
  }

  c_id
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_destroy(c_id: c_int) {
  if let Some(map) = HTTP2_MAP.get() {
    map.lock().unwrap().remove(&c_id);
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_end(c_stream_id: c_int) {
  if let Some(map) = HTTP2_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http2Stream>>> = {
      let streams: MutexGuard<'_, Http2Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let mut stream_lock: std::sync::MutexGuard<'_, Http2Stream> = stream.lock().unwrap();
        stream_lock.end();
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http2_end: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_logger(c_id: c_int, c_cb: Logger) {
  let http2_logger: Arc<Http2Logger> = Arc::new(move |level: &str, message: &str| {
    let c_level: CString = CString::new(level).unwrap();
    let c_message: CString = CString::new(message).unwrap();
    c_cb(c_id, c_level.as_ptr(), c_message.as_ptr());
  });

  if let Some(map) = HTTP2_MAP.get() {
    if let Some(http2) = map.lock().unwrap().get_mut(&c_id) {
      http2.logger(http2_logger);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_on(c_id: c_int, c_path: *const c_char, c_cb: Handler) {
  let path: &str = match unsafe { CStr::from_ptr(c_path) }.to_str() {
    Ok(s) => s,
    Err(_) => {
      println!("[Arnelify Server]: Rust FFI error in http2_on: Invalid UTF-8 in 'c_path'.");
      return;
    }
  };

  let http2_handler: Arc<Http2Handler> = Arc::new(
    move |ctx: Arc<Mutex<Http2Ctx>>, stream: Arc<Mutex<Http2Stream>>| {
      let stream_id: i32 = HTTP2_STREAM_ID.fetch_add(1, Ordering::Relaxed);

      HTTP2_STREAMS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
        .insert(stream_id, stream);

      let json: String = {
        let ctx_lock: MutexGuard<'_, Http2Ctx> = ctx.lock().unwrap();
        serde_json::to_string(&*ctx_lock).unwrap()
      };

      let c_json: CString = CString::new(json).unwrap();
      c_cb(c_id, stream_id, c_json.as_ptr());

      if let Some(map) = HTTP2_STREAMS.get() {
        map.lock().unwrap().remove(&stream_id);
      }
    },
  );

  if let Some(map) = HTTP2_MAP.get() {
    if let Some(http2) = map.lock().unwrap().get_mut(&c_id) {
      http2.on(path, http2_handler);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_push_bytes(
  c_stream_id: c_int,
  c_bytes: *const c_char,
  c_bytes_len: c_int,
  c_is_attachment: c_int,
) {
  if let Some(map) = HTTP2_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http2Stream>>> = {
      let streams: MutexGuard<'_, Http2Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let is_attachment: bool = c_is_attachment == 1;
        if c_bytes.is_null() || 0 >= c_bytes_len {
          let mut stream_lock: std::sync::MutexGuard<'_, Http2Stream> = stream.lock().unwrap();
          stream_lock.push_bytes(&[], is_attachment);
          return;
        }

        let bytes: &[u8] =
          unsafe { slice::from_raw_parts(c_bytes as *const u8, c_bytes_len as usize) };
        let mut stream_lock: std::sync::MutexGuard<'_, Http2Stream> = stream.lock().unwrap();
        stream_lock.push_bytes(bytes, is_attachment);
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http2_push_bytes: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_push_file(
  c_stream_id: c_int,
  c_file_path: *const c_char,
  c_is_attachment: c_int,
) {
  if let Some(map) = HTTP2_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http2Stream>>> = {
      let streams: MutexGuard<'_, Http2Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let is_attachment: bool = c_is_attachment == 1;
        let file_path: &str = match unsafe { CStr::from_ptr(c_file_path) }.to_str() {
          Ok(s) => s,
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http2_push_file: Invalid UTF-8 in 'c_file_path'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, Http2Stream> = stream.lock().unwrap();
        stream_lock.push_file(file_path, is_attachment);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http2_push_file: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_push_json(
  c_stream_id: c_int,
  c_json: *const c_char,
  c_is_attachment: c_int,
) {
  if let Some(map) = HTTP2_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http2Stream>>> = {
      let streams: MutexGuard<'_, Http2Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let is_attachment: bool = c_is_attachment == 1;
        let json: JSON = match unsafe { CStr::from_ptr(c_json) }.to_str() {
          Ok(s) => match serde_json::from_str(s) {
            Ok(json) => json,
            Err(_) => {
              println!(
                "[Arnelify Server]: Rust FFI error in http2_push_json: Invalid JSON in 'c_json'."
              );
              return;
            }
          },
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http2_push_json: Invalid UTF-8 in 'c_json'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, Http2Stream> = stream.lock().unwrap();
        stream_lock.push_json(&json, is_attachment);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http2_push_json: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_set_code(c_stream_id: c_int, c_code: c_int) {
  if let Some(map) = HTTP2_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http2Stream>>> = {
      let streams: MutexGuard<'_, Http2Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let mut stream_lock: std::sync::MutexGuard<'_, Http2Stream> = stream.lock().unwrap();
        stream_lock.set_code(c_code as u16);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http2_set_code: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_set_compression(c_stream_id: c_int, c_compression: *const c_char) {
  if let Some(map) = HTTP2_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http2Stream>>> = {
      let streams: MutexGuard<'_, Http2Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let compression: &str = match unsafe { CStr::from_ptr(c_compression) }.to_str() {
          Ok(s) => s,
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http2_set_compression: Invalid UTF-8 in 'c_compression'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, Http2Stream> = stream.lock().unwrap();
        if compression.len() > 0 {
          stream_lock.set_compression(Some(String::from(compression)));
          return;
        }

        stream_lock.set_compression(None);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http2_set_compression: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_set_headers(c_stream_id: c_int, c_headers: *const c_char) {
  if let Some(map) = HTTP2_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http2Stream>>> = {
      let streams: MutexGuard<'_, Http2Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let json: Vec<JSON> = match unsafe { CStr::from_ptr(c_headers) }.to_str() {
          Ok(s) => match serde_json::from_str(s) {
            Ok(json) => json,
            Err(_) => {
              println!(
                "[Arnelify Server]: Rust FFI error in http2_set_headers: Invalid JSON in 'c_headers'."
              );
              return;
            }
          },
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http2_set_headers: Invalid UTF-8 in 'c_headers'."
            );
            return;
          }
        };

        let mut headers: Vec<(String, String)> = Vec::new();
        for header in json {
          if let JSON::Object(pair) = header {
            for (key, value) in pair {
              let value = match value {
                JSON::String(s) => s,
                JSON::Number(n) => n.to_string(),
                JSON::Bool(b) => b.to_string(),
                _ => continue,
              };
              headers.push((key, value));
            }
          }
        }

        let mut stream_lock: std::sync::MutexGuard<'_, Http2Stream> = stream.lock().unwrap();
        stream_lock.set_headers(headers);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http2_set_headers: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn ws_close(c_stream_id: c_int) {
  if let Some(map) = WS_STREAMS.get() {
    let stream = {
      let streams: MutexGuard<'_, WebSocketStreams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let mut stream_lock: std::sync::MutexGuard<'_, WebSocketStream> = stream.lock().unwrap();
        stream_lock.close();
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in ws_close: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn ws_create(c_opts: *const c_char) -> c_int {
  let opts: JSON = match unsafe { CStr::from_ptr(c_opts) }.to_str() {
    Ok(s) => match serde_json::from_str(s) {
      Ok(json) => json,
      Err(_) => {
        println!("[Arnelify Server]: Rust FFI error in ws_create: Invalid JSON in 'c_opts'.");
        return 0;
      }
    },
    Err(_) => {
      println!("[Arnelify Server]: Rust FFI error in ws_create: Invalid UTF-8 in 'c_opts'.");
      return 0;
    }
  };

  let map: &Mutex<HashMap<c_int, Arc<WebSocket>>> =
    WS_MAP.get_or_init(|| Mutex::new(HashMap::new()));
  let id: &Mutex<c_int> = WS_ID.get_or_init(|| Mutex::new(0));
  let c_id: c_int = {
    let mut c: MutexGuard<'_, c_int> = id.lock().unwrap();
    *c += 1;
    *c
  };

  let ws_opts: WebSocketOpts = WebSocketOpts {
    block_size_kb: get_usize(&opts, "block_size_kb"),
    compression: get_bool(&opts, "compression"),
    handshake_timeout: get_u64(&opts, "handshake_timeout"),
    max_message_size_kb: get_u64(&opts, "max_message_size_kb"),
    ping_timeout: get_u64(&opts, "ping_timeout"),
    port: get_u16(&opts, "port"),
    send_timeout: get_u64(&opts, "send_timeout"),
    thread_limit: get_u64(&opts, "thread_limit"),
  };

  {
    let ws: WebSocket = WebSocket::new(ws_opts);
    map.lock().unwrap().insert(c_id, Arc::new(ws));
  }

  c_id
}

#[unsafe(no_mangle)]
pub extern "C" fn ws_destroy(c_id: c_int) {
  if let Some(map) = WS_MAP.get() {
    map.lock().unwrap().remove(&c_id);
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn ws_logger(c_id: c_int, c_cb: Logger) {
  let ws_logger: Arc<WebSocketLogger> = Arc::new(move |level, message| {
    let c_level: CString = CString::new(level).unwrap();
    let c_message: CString = CString::new(message).unwrap();
    c_cb(c_id, c_level.as_ptr(), c_message.as_ptr());
  });

  if let Some(map) = WS_MAP.get() {
    if let Some(ws) = map.lock().unwrap().get_mut(&c_id) {
      ws.logger(ws_logger);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn ws_on(c_id: c_int, c_topic: *const c_char, c_cb: HandlerWithTransport) {
  let topic: &str = match unsafe { CStr::from_ptr(c_topic) }.to_str() {
    Ok(s) => s,
    Err(_) => {
      println!("[Arnelify Server]: Rust FFI error in ws_on: Invalid UTF-8 in 'c_topic'.");
      return;
    }
  };

  let ws_handler: Arc<WebSocketHandler> = Arc::new(
    move |ctx: Arc<Mutex<WebSocketCtx>>,
          bytes: Arc<Mutex<WebSocketBytes>>,
          stream: Arc<Mutex<WebSocketStream>>| {
      let c_stream_id: i32 = WS_STREAM_ID.fetch_add(1, Ordering::Relaxed);

      WS_STREAMS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
        .insert(c_stream_id, stream);

      let json: String = {
        let ctx_lock: MutexGuard<'_, WebSocketCtx> = ctx.lock().unwrap();
        serde_json::to_string(&*ctx_lock).unwrap()
      };

      let bytes: Vec<u8> = {
        let bytes_lock: MutexGuard<'_, WebSocketBytes> = bytes.lock().unwrap();
        bytes_lock.clone()
      };

      let c_json: CString = CString::new(json).unwrap();
      let c_bytes: *const c_char = bytes.as_ptr() as *const c_char;

      c_cb(
        c_id,
        c_stream_id,
        c_json.as_ptr(),
        c_bytes,
        bytes.len() as c_int,
      );

      if let Some(map) = WS_STREAMS.get() {
        map.lock().unwrap().remove(&c_stream_id);
      }
    },
  );

  if let Some(map) = WS_MAP.get() {
    if let Some(ws) = map.lock().unwrap().get_mut(&c_id) {
      ws.on(topic, ws_handler);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn ws_push(
  c_stream_id: c_int,
  c_json: *const c_char,
  c_bytes: *const c_char,
  c_bytes_len: c_int,
) {
  if let Some(map) = WS_STREAMS.get() {
    let stream: Option<Arc<Mutex<WebSocketStream>>> = {
      let streams: MutexGuard<'_, WebSocketStreams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let json: JSON = match unsafe { CStr::from_ptr(c_json) }.to_str() {
          Ok(s) => match serde_json::from_str(s) {
            Ok(json) => json,
            Err(_) => {
              println!("[Arnelify Server]: Rust FFI error in ws_push: Invalid JSON in 'c_json'.");
              return;
            }
          },
          Err(_) => {
            println!("[Arnelify Server]: Rust FFI error in ws_push: Invalid UTF-8 in 'c_json'.");
            return;
          }
        };

        let bytes: &[u8] = if c_bytes.is_null() || c_bytes_len <= 0 {
          &[]
        } else {
          unsafe { std::slice::from_raw_parts(c_bytes as *const u8, c_bytes_len as usize) }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, WebSocketStream> = stream.lock().unwrap();
        stream_lock.push(&json, &bytes);
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in ws_push: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn ws_push_bytes(c_stream_id: c_int, c_bytes: *const c_char, c_bytes_len: c_int) {
  if let Some(map) = WS_STREAMS.get() {
    let stream: Option<Arc<Mutex<WebSocketStream>>> = {
      let streams: MutexGuard<'_, WebSocketStreams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let bytes: &[u8] = if c_bytes.is_null() || c_bytes_len <= 0 {
          &[]
        } else {
          unsafe { std::slice::from_raw_parts(c_bytes as *const u8, c_bytes_len as usize) }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, WebSocketStream> = stream.lock().unwrap();
        stream_lock.push_bytes(&bytes);
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in ws_push_bytes: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn ws_push_json(c_stream_id: c_int, c_json: *const c_char) {
  if let Some(map) = WS_STREAMS.get() {
    let stream: Option<Arc<Mutex<WebSocketStream>>> = {
      let streams: MutexGuard<'_, WebSocketStreams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let json: JSON = match unsafe { CStr::from_ptr(c_json) }.to_str() {
          Ok(s) => match serde_json::from_str(s) {
            Ok(json) => json,
            Err(_) => {
              println!(
                "[Arnelify Server]: Rust FFI error in ws_push_json: Invalid JSON in 'c_json'."
              );
              return;
            }
          },
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in ws_push_json: Invalid UTF-8 in 'c_json'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, WebSocketStream> = stream.lock().unwrap();
        stream_lock.push_json(&json);
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in ws_push_json: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn ws_start(c_id: c_int) {
  if let Some(map) = WS_MAP.get() {
    if let Some(ws) = map.lock().unwrap().get_mut(&c_id) {
      ws.start();
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn ws_stop(c_id: c_int) {
  if let Some(map) = WS_MAP.get() {
    if let Some(ws) = map.lock().unwrap().get_mut(&c_id) {
      ws.stop();
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_start(c_id: c_int) {
  if let Some(map) = HTTP2_MAP.get() {
    if let Some(http2) = map.lock().unwrap().get_mut(&c_id) {
      http2.start();
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http2_stop(c_id: c_int) {
  if let Some(map) = HTTP2_MAP.get() {
    if let Some(http2) = map.lock().unwrap().get_mut(&c_id) {
      http2.stop();
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_add_header(
  c_stream_id: c_int,
  c_key: *const c_char,
  c_value: *const c_char,
) {
  if let Some(map) = HTTP3_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http3Stream>>> = {
      let streams: MutexGuard<'_, Http3Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let key: &str = match unsafe { CStr::from_ptr(c_key) }.to_str() {
          Ok(s) => s,
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http3_add_header: Invalid UTF-8 in 'c_key'."
            );
            return;
          }
        };

        let value: &str = match unsafe { CStr::from_ptr(c_value) }.to_str() {
          Ok(s) => s,
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http3_add_header: Invalid UTF-8 in 'c_value'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, Http3Stream> = stream.lock().unwrap();
        stream_lock.add_header(key, value);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http3_add_header: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_create(c_opts: *const c_char) -> c_int {
  let opts: JSON = match unsafe { CStr::from_ptr(c_opts) }.to_str() {
    Ok(s) => match serde_json::from_str(s) {
      Ok(json) => json,
      Err(_) => {
        println!("[Arnelify Server]: Rust FFI error in http3_create: Invalid JSON in 'c_opts'.");
        return 0;
      }
    },
    Err(_) => {
      println!("[Arnelify Server]: Rust FFI error in http3_create: Invalid UTF-8 in 'c_opts'.");
      return 0;
    }
  };

  let map: &Mutex<HashMap<c_int, Arc<Http3>>> =
    HTTP3_MAP.get_or_init(|| Mutex::new(HashMap::new()));
  let id: &Mutex<c_int> = HTTP3_ID.get_or_init(|| Mutex::new(0));
  let c_id: c_int = {
    let mut c: MutexGuard<'_, c_int> = id.lock().unwrap();
    *c += 1;
    *c
  };

  let http3_opts: Http3Opts = Http3Opts {
    allow_empty_files: get_bool(&opts, "allow_empty_files"),
    block_size_kb: get_usize(&opts, "block_size_kb"),
    cert_pem: get_str(&opts, "cert_pem"),
    charset: get_str(&opts, "charset"),
    compression: get_bool(&opts, "compression"),
    keep_alive: get_u8(&opts, "keep_alive"),
    keep_extensions: get_bool(&opts, "keep_extensions"),
    key_pem: get_str(&opts, "key_pem"),
    max_fields: get_u32(&opts, "max_fields"),
    max_fields_size_total_mb: get_usize(&opts, "max_fields_size_total_mb"),
    max_files: get_u32(&opts, "max_files"),
    max_files_size_total_mb: get_usize(&opts, "max_files_size_total_mb"),
    max_file_size_mb: get_usize(&opts, "max_file_size_mb"),
    port: get_u16(&opts, "port"),
    storage_path: get_str(&opts, "storage_path"),
    thread_limit: get_u64(&opts, "thread_limit"),
  };

  {
    let http3: Http3 = Http3::new(http3_opts);
    map.lock().unwrap().insert(c_id, Arc::new(http3));
  }

  c_id
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_destroy(c_id: c_int) {
  if let Some(map) = HTTP3_MAP.get() {
    map.lock().unwrap().remove(&c_id);
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_end(c_stream_id: c_int) {
  if let Some(map) = HTTP3_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http3Stream>>> = {
      let streams: MutexGuard<'_, Http3Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let mut stream_lock: std::sync::MutexGuard<'_, Http3Stream> = stream.lock().unwrap();
        stream_lock.end();
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http3_end: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_logger(c_id: c_int, c_cb: Logger) {
  let http3_logger: Arc<Http3Logger> = Arc::new(move |level: &str, message: &str| {
    let c_level: CString = CString::new(level).unwrap();
    let c_message: CString = CString::new(message).unwrap();
    c_cb(c_id, c_level.as_ptr(), c_message.as_ptr());
  });

  if let Some(map) = HTTP3_MAP.get() {
    if let Some(http3) = map.lock().unwrap().get_mut(&c_id) {
      http3.logger(http3_logger);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_on(c_id: c_int, c_path: *const c_char, c_cb: Handler) {
  let path: &str = match unsafe { CStr::from_ptr(c_path) }.to_str() {
    Ok(s) => s,
    Err(_) => {
      println!("[Arnelify Server]: Rust FFI error in http3_on: Invalid UTF-8 in 'c_path'.");
      return;
    }
  };

  let http3_handler: Arc<Http3Handler> = Arc::new(
    move |ctx: Arc<Mutex<Http3Ctx>>, stream: Arc<Mutex<Http3Stream>>| {
      let stream_id: i32 = HTTP3_STREAM_ID.fetch_add(1, Ordering::Relaxed);

      HTTP3_STREAMS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
        .insert(stream_id, stream);

      let json: String = {
        let ctx_lock: MutexGuard<'_, Http3Ctx> = ctx.lock().unwrap();
        serde_json::to_string(&*ctx_lock).unwrap()
      };

      let c_json: CString = CString::new(json).unwrap();
      c_cb(c_id, stream_id, c_json.as_ptr());

      if let Some(map) = HTTP3_STREAMS.get() {
        map.lock().unwrap().remove(&stream_id);
      }
    },
  );

  if let Some(map) = HTTP3_MAP.get() {
    if let Some(http3) = map.lock().unwrap().get_mut(&c_id) {
      http3.on(path, http3_handler);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_push_bytes(
  c_stream_id: c_int,
  c_bytes: *const c_char,
  c_bytes_len: c_int,
  c_is_attachment: c_int,
) {
  if let Some(map) = HTTP3_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http3Stream>>> = {
      let streams: MutexGuard<'_, Http3Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let is_attachment: bool = c_is_attachment == 1;
        if c_bytes.is_null() || 0 >= c_bytes_len {
          let mut stream_lock: std::sync::MutexGuard<'_, Http3Stream> = stream.lock().unwrap();
          stream_lock.push_bytes(&[], is_attachment);
          return;
        }

        let bytes: &[u8] =
          unsafe { slice::from_raw_parts(c_bytes as *const u8, c_bytes_len as usize) };
        let mut stream_lock: std::sync::MutexGuard<'_, Http3Stream> = stream.lock().unwrap();
        stream_lock.push_bytes(bytes, is_attachment);
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http3_push_bytes: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_push_file(
  c_stream_id: c_int,
  c_file_path: *const c_char,
  c_is_attachment: c_int,
) {
  if let Some(map) = HTTP3_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http3Stream>>> = {
      let streams: MutexGuard<'_, Http3Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let is_attachment: bool = c_is_attachment == 1;
        let file_path: &str = match unsafe { CStr::from_ptr(c_file_path) }.to_str() {
          Ok(s) => s,
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http3_push_file: Invalid UTF-8 in 'c_file_path'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, Http3Stream> = stream.lock().unwrap();
        stream_lock.push_file(file_path, is_attachment);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http3_push_file: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_push_json(
  c_stream_id: c_int,
  c_json: *const c_char,
  c_is_attachment: c_int,
) {
  if let Some(map) = HTTP3_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http3Stream>>> = {
      let streams: MutexGuard<'_, Http3Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let is_attachment: bool = c_is_attachment == 1;
        let json: JSON = match unsafe { CStr::from_ptr(c_json) }.to_str() {
          Ok(s) => match serde_json::from_str(s) {
            Ok(json) => json,
            Err(_) => {
              println!(
                "[Arnelify Server]: Rust FFI error in http3_push_json: Invalid JSON in 'c_json'."
              );
              return;
            }
          },
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http3_push_json: Invalid UTF-8 in 'c_json'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, Http3Stream> = stream.lock().unwrap();
        stream_lock.push_json(&json, is_attachment);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http3_push_json: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_set_code(c_stream_id: c_int, c_code: c_int) {
  if let Some(map) = HTTP3_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http3Stream>>> = {
      let streams: MutexGuard<'_, Http3Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let mut stream_lock: std::sync::MutexGuard<'_, Http3Stream> = stream.lock().unwrap();
        stream_lock.set_code(c_code as u16);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http3_set_code: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_set_compression(c_stream_id: c_int, c_compression: *const c_char) {
  if let Some(map) = HTTP3_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http3Stream>>> = {
      let streams: MutexGuard<'_, Http3Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let compression: &str = match unsafe { CStr::from_ptr(c_compression) }.to_str() {
          Ok(s) => s,
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http3_set_compression: Invalid UTF-8 in 'c_compression'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, Http3Stream> = stream.lock().unwrap();
        if compression.len() > 0 {
          stream_lock.set_compression(Some(String::from(compression)));
          return;
        }

        stream_lock.set_compression(None);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http3_set_compression: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_set_headers(c_stream_id: c_int, c_headers: *const c_char) {
  if let Some(map) = HTTP3_STREAMS.get() {
    let stream: Option<Arc<Mutex<Http3Stream>>> = {
      let streams: MutexGuard<'_, Http3Streams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let json: Vec<JSON> = match unsafe { CStr::from_ptr(c_headers) }.to_str() {
          Ok(s) => match serde_json::from_str(s) {
            Ok(json) => json,
            Err(_) => {
              println!(
                "[Arnelify Server]: Rust FFI error in http3_set_headers: Invalid JSON in 'c_headers'."
              );
              return;
            }
          },
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in http3_set_headers: Invalid UTF-8 in 'c_headers'."
            );
            return;
          }
        };

        let mut headers: Vec<(String, String)> = Vec::new();
        for header in json {
          if let JSON::Object(pair) = header {
            for (key, value) in pair {
              let value = match value {
                JSON::String(s) => s,
                JSON::Number(n) => n.to_string(),
                JSON::Bool(b) => b.to_string(),
                _ => continue,
              };
              headers.push((key, value));
            }
          }
        }

        let mut stream_lock: std::sync::MutexGuard<'_, Http3Stream> = stream.lock().unwrap();
        stream_lock.set_headers(headers);
        return;
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in http3_set_headers: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_start(c_id: c_int) {
  if let Some(map) = HTTP3_MAP.get() {
    if let Some(http3) = map.lock().unwrap().get_mut(&c_id) {
      http3.start();
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn http3_stop(c_id: c_int) {
  if let Some(map) = HTTP3_MAP.get() {
    if let Some(http3) = map.lock().unwrap().get_mut(&c_id) {
      http3.stop();
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn wt_close(c_stream_id: c_int) {
  if let Some(map) = WT_STREAMS.get() {
    let stream: Option<Arc<Mutex<WebTransportStream>>> = {
      let streams: MutexGuard<'_, WebTransportStreams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let mut stream_lock: std::sync::MutexGuard<'_, WebTransportStream> = stream.lock().unwrap();
        stream_lock.close();
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in wt_close: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn wt_create(c_opts: *const c_char) -> c_int {
  let opts: JSON = match unsafe { CStr::from_ptr(c_opts) }.to_str() {
    Ok(s) => match serde_json::from_str(s) {
      Ok(json) => json,
      Err(_) => {
        println!("[Arnelify Server]: Rust FFI error in wt_create: Invalid JSON in 'c_opts'.");
        return 0;
      }
    },
    Err(_) => {
      println!("[Arnelify Server]: Rust FFI error in wt_create: Invalid UTF-8 in 'c_opts'.");
      return 0;
    }
  };

  let map: &Mutex<HashMap<c_int, Arc<WebTransport>>> =
    WT_MAP.get_or_init(|| Mutex::new(HashMap::new()));
  let id: &Mutex<c_int> = WT_ID.get_or_init(|| Mutex::new(0));
  let c_id: c_int = {
    let mut c: MutexGuard<'_, c_int> = id.lock().unwrap();
    *c += 1;
    *c
  };

  let wt_opts: WebTransportOpts = WebTransportOpts {
    block_size_kb: get_usize(&opts, "block_size_kb"),
    cert_pem: get_str(&opts, "cert_pem"),
    compression: get_bool(&opts, "compression"),
    handshake_timeout: get_u64(&opts, "handshake_timeout"),
    key_pem: get_str(&opts, "key_pem"),
    max_message_size_kb: get_u64(&opts, "max_message_size_kb"),
    ping_timeout: get_u64(&opts, "ping_timeout"),
    port: get_u16(&opts, "port"),
    send_timeout: get_u64(&opts, "send_timeout"),
    thread_limit: get_u64(&opts, "thread_limit"),
  };

  {
    let wt: WebTransport = WebTransport::new(wt_opts);
    map.lock().unwrap().insert(c_id, Arc::new(wt));
  }

  c_id
}

#[unsafe(no_mangle)]
pub extern "C" fn wt_destroy(c_id: c_int) {
  if let Some(map) = WT_MAP.get() {
    map.lock().unwrap().remove(&c_id);
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn wt_logger(c_id: c_int, c_cb: Logger) {
  let wt_logger: Arc<WebTransportLogger> = Arc::new(move |level, message| {
    let c_level: CString = CString::new(level).unwrap();
    let c_message: CString = CString::new(message).unwrap();
    c_cb(c_id, c_level.as_ptr(), c_message.as_ptr());
  });

  if let Some(map) = WT_MAP.get() {
    if let Some(wt) = map.lock().unwrap().get_mut(&c_id) {
      wt.logger(wt_logger);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn wt_on(c_id: c_int, c_topic: *const c_char, c_cb: HandlerWithTransport) {
  let topic: &str = match unsafe { CStr::from_ptr(c_topic) }.to_str() {
    Ok(s) => s,
    Err(_) => {
      println!("[Arnelify Server]: Rust FFI error in wt_on: Invalid UTF-8 in 'c_topic'.");
      return;
    }
  };

  let wt_handler: Arc<WebTransportHandler> = Arc::new(
    move |ctx: Arc<Mutex<WebTransportCtx>>,
          bytes: Arc<Mutex<WebTransportBytes>>,
          stream: Arc<Mutex<WebTransportStream>>| {
      let c_stream_id: i32 = WT_STREAM_ID.fetch_add(1, Ordering::Relaxed);

      WT_STREAMS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
        .insert(c_stream_id, stream);

      let json: String = {
        let ctx_lock: MutexGuard<'_, WebTransportCtx> = ctx.lock().unwrap();
        serde_json::to_string(&*ctx_lock).unwrap()
      };

      let bytes: Vec<u8> = {
        let bytes_lock: MutexGuard<'_, WebTransportBytes> = bytes.lock().unwrap();
        bytes_lock.clone()
      };

      let c_json: CString = CString::new(json).unwrap();
      let c_bytes: *const c_char = bytes.as_ptr() as *const c_char;

      c_cb(
        c_id,
        c_stream_id,
        c_json.as_ptr(),
        c_bytes,
        bytes.len() as c_int,
      );

      if let Some(map) = WT_STREAMS.get() {
        map.lock().unwrap().remove(&c_stream_id);
      }
    },
  );

  if let Some(map) = WT_MAP.get() {
    if let Some(wt) = map.lock().unwrap().get_mut(&c_id) {
      wt.on(topic, wt_handler);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn wt_push(
  c_stream_id: c_int,
  c_json: *const c_char,
  c_bytes: *const c_char,
  c_bytes_len: c_int,
) {
  if let Some(map) = WT_STREAMS.get() {
    let stream: Option<Arc<Mutex<WebTransportStream>>> = {
      let streams: MutexGuard<'_, WebTransportStreams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let json: JSON = match unsafe { CStr::from_ptr(c_json) }.to_str() {
          Ok(s) => match serde_json::from_str(s) {
            Ok(json) => json,
            Err(_) => {
              println!("[Arnelify Server]: Rust FFI error in wt_push: Invalid JSON in 'c_json'.");
              return;
            }
          },
          Err(_) => {
            println!("[Arnelify Server]: Rust FFI error in wt_push: Invalid UTF-8 in 'c_json'.");
            return;
          }
        };

        let bytes: &[u8] = if c_bytes.is_null() || c_bytes_len <= 0 {
          &[]
        } else {
          unsafe { std::slice::from_raw_parts(c_bytes as *const u8, c_bytes_len as usize) }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, WebTransportStream> = stream.lock().unwrap();
        stream_lock.push(&json, &bytes);
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in wt_push: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn wt_push_bytes(c_stream_id: c_int, c_bytes: *const c_char, c_bytes_len: c_int) {
  if let Some(map) = WT_STREAMS.get() {
    let stream: Option<Arc<Mutex<WebTransportStream>>> = {
      let streams: MutexGuard<'_, WebTransportStreams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let bytes: &[u8] = if c_bytes.is_null() || c_bytes_len <= 0 {
          &[]
        } else {
          unsafe { std::slice::from_raw_parts(c_bytes as *const u8, c_bytes_len as usize) }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, WebTransportStream> = stream.lock().unwrap();
        stream_lock.push_bytes(&bytes);
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in wt_push_bytes: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn wt_push_json(c_stream_id: c_int, c_json: *const c_char) {
  if let Some(map) = WT_STREAMS.get() {
    let stream: Option<Arc<Mutex<WebTransportStream>>> = {
      let streams: MutexGuard<'_, WebTransportStreams> = map.lock().unwrap();
      streams.get(&c_stream_id).cloned()
    };

    match stream {
      Some(stream) => {
        let json: JSON = match unsafe { CStr::from_ptr(c_json) }.to_str() {
          Ok(s) => match serde_json::from_str(s) {
            Ok(json) => json,
            Err(_) => {
              println!(
                "[Arnelify Server]: Rust FFI error in wt_push_json: Invalid JSON in 'c_json'."
              );
              return;
            }
          },
          Err(_) => {
            println!(
              "[Arnelify Server]: Rust FFI error in wt_push_json: Invalid UTF-8 in 'c_json'."
            );
            return;
          }
        };

        let mut stream_lock: std::sync::MutexGuard<'_, WebTransportStream> = stream.lock().unwrap();
        stream_lock.push_json(&json);
      }
      None => {
        println!("[Arnelify Server]: Rust FFI error in wt_push_json: No stream found.");
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn wt_start(c_id: c_int) {
  if let Some(map) = WT_MAP.get() {
    if let Some(wt) = map.lock().unwrap().get_mut(&c_id) {
      wt.start();
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn wt_stop(c_id: c_int) {
  if let Some(map) = WT_MAP.get() {
    if let Some(wt) = map.lock().unwrap().get_mut(&c_id) {
      wt.stop();
    }
  }
}