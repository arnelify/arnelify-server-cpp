#include <iostream>

#include "json.h"
#include "lib.hpp"

int main() {
  WebSocketOpts opts(
      /* block_size_kb */ 64,
      /* compression */ true,
      /* handshake_timeout */ 30,
      /* max_message_size_kb */ 60,
      /* ping_timeout */ 30,
      /* port */ 4433,
      /* send_timeout */ 60,
      /* thread_limit */ 4);

  WebSocket ws(opts);
  WebSocketLogger ws_logger = [](const std::string& level,
                                 const std::string& message) -> void {
    std::cout << "[Arnelify Server]: " << message << std::endl;
  };

  ws.logger(ws_logger);

  WebSocketHandler ws_handler = [](WebSocketCtx& ctx, WebSocketBytes& bytes,
                                   WebSocketStream& stream) -> void {
    stream.push(ctx, bytes);
    stream.close();
  };

  ws.on("connect", ws_handler);
  ws.start();
}