FROM ghcr.io/cross-rs/arm-unknown-linux-musleabihf:edge

RUN echo "nameserver 8.8.8.8" | tee /etc/resolv.conf > /dev/null

RUN dpkg --add-architecture armhf
RUN apt-get update -y

# Standard Kobo toolchain requirements
# RUN apt-get install -y build-essential crossbuild-essential-armhf cross-gcc-dev g++-arm-linux-gnueabihf gcc-arm-linux-gnueabihf nasm

# Libs we need
RUN apt-get install -y libevdev-dev:armhf
RUN apt-get install -y python3

# Configure rust to static compile
ENV RUSTFLAGS='-C target-feature=+crt-static'