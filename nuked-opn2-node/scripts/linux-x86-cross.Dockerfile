FROM rustembedded/cross:i686-unknown-linux-gnu

RUN apt-get update && apt-get install -y libclang-dev gcc-multilib
