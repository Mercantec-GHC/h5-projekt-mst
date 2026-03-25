#include "tcp.hpp"
#include "errno_shim.hpp"
#include <arpa/inet.h>
#include <cstdlib>
#include <expected>
#include <netdb.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string>
#include <sys/epoll.h>
#include <sys/socket.h>
#include <unistd.h>

namespace mst {

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

    // enable immediate reuse of socket address
    // see socket(7) about SO_REUSEADDR
    // > Argument is an integer boolean flag.
    int reuse_address = true;
    if (::setsockopt(socket_fd,
            SOL_SOCKET,
            SO_REUSEADDR,
            &reuse_address,
            sizeof(reuse_address))) {
        return std::unexpected(errno_shim("could not configure socket"));
    }

    if (::bind(socket_fd, (struct sockaddr*)&address, sizeof(address)) < 0) {
        return std::unexpected(errno_shim("could not bind"));
    }

    if (::listen(socket_fd, 0) < 0) {
        return std::unexpected(errno_shim("could not listen"));
    }

    return TcpListener(address, socket_fd);
}

auto TcpListener::accept() -> Result<TcpConnection>
{
    socklen_t size = sizeof(this->address);
    int client = ::accept(this->fd, (struct sockaddr*)&address, &size);
    if (client < 0) {
        return std::unexpected(errno_shim("could not accept"));
    }

    return TcpConnection(*this, client);
}

}
