FROM alpine:3.23
RUN apk add --no-cache curl build-base sdl3-dev

RUN curl https://sh.rustup.rs -sSf \
    | sh -s -- --default-toolchain stable -y

ENV PATH="/root/.cargo/bin:${PATH}"

