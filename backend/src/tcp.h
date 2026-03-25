#pragma once

#include <cstdint>
#include <expected>
#include <netinet/in.h>
#include <string>
#include <unistd.h>

namespace mst {

template <typename T> using Result = std::expected<T, std::string>;

class TcpConnection;

namespace event {
    enum Variant { Listener, Connection };
    typedef union {
        int listener_fd;
        TcpConnection* connection;
    } Data;
    struct Event {
        Variant variant;
        Data data;
    };

    auto make_listener_event(int fd) -> Event*;
    auto make_connection_event(TcpConnection connection) -> Event*;
}

class TcpListener;

class TcpConnection {
public:
    auto write(uint8_t* buffer, size_t len) -> Result<size_t>;
    auto read(uint8_t* buffer, size_t len) -> Result<size_t>;
    TcpConnection(TcpListener&, int fd)
        : fd(fd) { };

private:
    int fd;
    event::Event event;
};

class TcpListener {

public:
    static auto bind(const std::string& host, uint16_t port)
        -> Result<TcpListener>;
    auto start() -> Result<void>;

private:
    TcpListener(int epoll_fd, sockaddr_in address)
        : epoll_fd(epoll_fd)
        , address(address) { };
    int epoll_fd;
    sockaddr_in address;
};
}
