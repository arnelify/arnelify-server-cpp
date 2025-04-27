FROM gcc:14.2.0

WORKDIR /var/www/cpp
ENV TZ=Europe/Kiev

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install sudo nano clang ccache -y
RUN apt-get install libjsoncpp-dev -y

# apt install libnghttp3-dev libngtcp2-dev libssl-dev libngtcp2-crypto-gnutls-dev
EXPOSE 3001