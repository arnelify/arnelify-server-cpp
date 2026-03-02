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
      /* send_timeout */ 60,
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