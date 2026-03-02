FROM gcc:15.2.0

WORKDIR /var/www/cpp

ENV TZ=Europe/Kiev
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install sudo nano clang ccache libjsoncpp-dev -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
  | sh -s -- -y --no-modify-path --default-toolchain 1.92.0

EXPOSE 4433