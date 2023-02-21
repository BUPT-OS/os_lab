FROM ubuntu:20.04
ARG DEBIAN_FRONTEND=noninteractive
# 换源
RUN sed -i "s@http://.*archive.ubuntu.com@http://mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list
RUN sed -i "s@http://.*security.ubuntu.com@http://mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list
RUN apt-get update
# 安装git
RUN apt-get install git curl build-essential -y
RUN export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
RUN export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
COPY . /linked_list
WORKDIR /linked_list
CMD bash