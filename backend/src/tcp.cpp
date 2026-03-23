#include "tcp.h"
#include <arpa/inet.h>
#include <errno.h>
#include <expected>
#include <format>
#include <netdb.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <string>
#include <string_view>
#include <sys/epoll.h>
#include <sys/socket.h>
#include <unistd.h>

namespace mst {
auto errno_shim(std::string_view message) -> std::string
{
    auto x = strerror(errno);
    return std::format("{} ({})", message, x);
}

auto TcpConnection::write(uint8_t* buffer, size_t len) -> Result<size_t>
{
    ssize_t bytes_written = ::write(this->fd, buffer, len);
    if (bytes_written < 0) {
        return std::unexpected(errno_shim("could not write bytes"));
    }
    return bytes_written;
}

auto TcpConnection::read(uint8_t* buffer, size_t len) -> Result<size_t>
{
    ssize_t bytes_read = ::recv(this->fd, buffer, len, 0);
    if (bytes_read < 0) {
        return std::unexpected(errno_shim("could not read bytes"));
    }
    return bytes_read;
}

auto TcpListener::bind(const std::string& host, uint16_t port)
    -> Result<TcpListener>
{
    int socket_fd = 0;
    if ((socket_fd = ::socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        return std::unexpected(errno_shim("could not get socket"));
    }

    struct sockaddr_in address = {
        .sin_family = AF_INET,
        .sin_port = htons(port),
        .sin_addr = in_addr { .s_addr = inet_addr(host.c_str()) },
        .sin_zero = { },
    };

    if (::bind(socket_fd, (struct sockaddr*)&address, sizeof(address)) < 0) {
        return std::unexpected(errno_shim("could not bind"));
    }

    if (::listen(socket_fd, 0) < 0) {
        return std::unexpected(errno_shim("could not listen"));
    }

    auto epoll_fd = ::epoll_create(0);

    auto events_accepted = epoll_event { .events = EPOLLIN, .data = { } };
    if (::epoll_ctl(epoll_fd, EPOLL_CTL_ADD, socket_fd, &events_accepted) < 0) {
        return std::unexpected(errno_shim("could not connect to epoll"));
    }

    return TcpListener(socket_fd, epoll_fd, address);
}

auto TcpListener::accept() -> Result<TcpConnection>
{
    socklen_t size = sizeof(address);
    int socket = ::accept(this->listener_fd, (struct sockaddr*)&address, &size);
    if (socket < 0) {
        return std::unexpected(errno_shim("could not accept"));
    }
    return TcpConnection(*this, socket);
}

}
