FROM rustembedded/cross:aarch64-unknown-linux-gnu

RUN apt-get update && \
    apt-get install clang-3.9 gcc-aarch64-linux-gnu libc6-dev-i386 -y
