FROM rust:latest

RUN apt update \
  && apt install sudo

RUN sudo apt update \
  && sudo apt install -y git clang

RUN rustup update \
  && rustup component add rustfmt \
  && rustup component add clippy \
  && cargo install sccache --locked \
  && cargo install cargo-udeps --locked \
  && cargo install sqlx-cli --no-default-features --features rustls,postgres \
  && echo 'export "RUSTC_WRAPPER=$(which sccache)"' >>~/.bashrc

RUN wget https://github.com/rui314/mold/releases/download/v2.4.1/mold-2.4.1-x86_64-linux.tar.gz -P /tmp \
  && tar -zxvf /tmp/mold-2.4.1-x86_64-linux.tar.gz -C /tmp \
  && cp /tmp/mold-2.4.1-x86_64-linux/bin/mold /usr/local/bin \
  && cp /tmp/mold-2.4.1-x86_64-linux/bin/ld.mold /usr/local/bin

ENV SCCACHE_DIR=/var/www/.cache/sccache
ENV SCCACHE_CACHE_SIZE="20G"

ENV TZ Asia/Tokyo

WORKDIR /var/www