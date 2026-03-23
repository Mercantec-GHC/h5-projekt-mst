#pragma once

#include <cstdint>
#include <expected>
#include <netinet/in.h>
#include <string>
#include <vector>

namespace mst {
template <typename T> using Result = std::expected<T, std::string>;

class TcpListener;

class TcpConnection {
public:
    auto write(uint8_t* buffer, size_t len) -> Result<size_t>;
    auto read(uint8_t* buffer, size_t len) -> Result<size_t>;
    TcpConnection(TcpListener&, int fd)
        : fd(fd) { };

private:
    int fd;
};

class TcpListener {

public:
    static auto bind(const std::string& host, uint16_t port)
        -> Result<TcpListener>;
    auto accept() -> Result<TcpConnection>;

private:
    TcpListener(int listener_fd, int epoll_fd, sockaddr_in address)
        : listener_fd(listener_fd)
        , epoll_fd(epoll_fd)
        , address(address) { };
    int listener_fd;
    int epoll_fd;
    sockaddr_in address;
    std::vector<int> epoll_fds;
};
}
