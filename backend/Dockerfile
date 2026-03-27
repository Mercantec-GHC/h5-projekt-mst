FROM docker.io/alpine:3.23.3 AS builder
WORKDIR /workspace
RUN apk add build-base
COPY Makefile .
COPY src src
COPY tests tests
RUN make RELEASE=1 build/backend

FROM docker.io/alpine:3.23.3 AS runner
WORKDIR /workspace
RUN apk add libstdc++
COPY --from=builder /workspace/build/backend /workspace
CMD ["/workspace/backend"] 
