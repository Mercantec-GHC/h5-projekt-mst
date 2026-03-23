#pragma once

#include "tcp.hpp"
#include <cstdint>
#include <expected>
#include <netinet/in.h>
#include <string>

namespace mst {
template <typename T> using Result = std::expected<T, std::string>;

class HttpServer;

class HttpDaemon {

public:
    HttpDaemon(HttpServer&, TcpConnection connection)
        : connection(connection) { };

private:
    TcpConnection connection;
};

class HttpServer {

public:
    auto start() -> Result<void>;
    static auto bind(const std::string& host, uint16_t port)
        -> Result<HttpServer>;

private:
    HttpServer(TcpListener listener)
        : listener(listener) { };
    TcpListener listener;
};
}
