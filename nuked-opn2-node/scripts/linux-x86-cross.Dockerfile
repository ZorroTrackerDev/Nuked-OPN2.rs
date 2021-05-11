FROM rustembedded/cross:i686-unknown-linux-gnu

RUN apt-get update && \
    apt-get install clang-3.9 gcc-multilib -y
