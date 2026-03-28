FROM alpine:edge
RUN apk add --no-cache curl build-base clang22-extra-tools mosquitto-dev sdl3-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

