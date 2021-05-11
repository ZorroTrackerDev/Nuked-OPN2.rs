FROM rustembedded/cross:i686-pc-windows-gnu

RUN apt-get update && apt-get install -y libclang-dev gcc-multilib 
