FROM rustembedded/cross:x86_64-unknown-linux-gnu

RUN yum install dnf -y
RUN dnf install 'dnf-command(copr)' -y
RUN dnf copr enable alonid/llvm-3.9.0 -y 
RUN yum install clang-3.9.0 -y
ENV PATH="$PATH:/opt/llvm-3.9.0/bin"
ENV LIBCLANG_PATH=/opt/llvm-3.9.0/lib64/

