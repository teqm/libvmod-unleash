FROM debian:11

ARG DEBIAN_FRONTEND=noninteractive
ARG CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN apt-get update && apt-get -y install \
      curl \
      apt-transport-https \
      build-essential \
      gnupg \
      git-core \
      libssl-dev \
      libclang-dev \
      pkg-config \
      python3-docutils

RUN curl -s https://packagecloud.io/install/repositories/varnishcache/varnish73/script.deb.sh | bash

RUN apt-get -y install \
      varnish-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install cargo-deb

COPY . /tmp/libvmod-unleash

RUN cd /tmp/libvmod-unleash && \
    cargo deb
