#include <iostream>

#include "json.h"
#include "lib.hpp"

int main() {
  WebSocketOpts opts(
      /* block_size_kb */ 64,
      /* compression */ false,
      /* max_message_size_kb */ 64,
      /* ping_timeout */ 15,
      /* port */ 4433,
      /* rate_limit */ 5,
      /* read_timeout */ 30,
      /* send_timeout */ 30,
      /* thread_limit */ 4);

  WebSocket ws(opts);
  WebSocketLogger ws_logger = [](const std::string& _level,
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