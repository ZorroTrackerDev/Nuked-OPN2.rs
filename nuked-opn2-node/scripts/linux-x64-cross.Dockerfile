FROM rust:1

RUN apt-get update && apt-get install -y \
    libclang-dev