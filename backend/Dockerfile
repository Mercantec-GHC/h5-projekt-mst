FROM alpine:3.23 AS builder
WORKDIR /workspace
RUN apk add build-base mosquitto-dev
COPY Makefile .
COPY src src
COPY tests tests
RUN make RELEASE=1 build/backend

FROM alpine:3.23 AS runner
WORKDIR /workspace
RUN apk add libstdc++ mosquitto-libs
COPY --from=builder /workspace/build/backend /workspace
CMD ["/workspace/backend"] 

