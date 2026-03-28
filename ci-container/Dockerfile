FROM alpine:edge
RUN apk add --no-cache curl build-base clang22-extra-tools mosquitto-dev sdl3-dev

RUN curl https://sh.rustup.rs -sSf \
    | sh -s -- --default-toolchain stable -y

ENV PATH="/root/.cargo/bin:${PATH}"

