FROM rustembedded/cross:x86_64-unknown-linux-gnu

RUN apt-get update && apt-get install -y libclang-dev
