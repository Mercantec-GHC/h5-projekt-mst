#pragma once

#include "result.hpp"
#include <cstdint>
#include <netinet/in.h>
#include <string>
#include <unistd.h>

namespace mst {

class TcpListener;

class TcpConnection {
public:
    auto write(uint8_t* buffer, size_t len) -> Result<size_t>;
    auto read(uint8_t* buffer, size_t len) -> Result<size_t>;
    TcpConnection(TcpListener&, int fd)
        : fd(fd) { };

    int fd;
};

class TcpListener {

public:
    static auto bind(const std::string& host, uint16_t port)
        -> Result<TcpListener>;
    auto accept() -> Result<TcpConnection>;
    int fd;

private:
    TcpListener(sockaddr_in address, int fd)
        : fd(fd)
        , address(address) { };
    sockaddr_in address;
};
}
