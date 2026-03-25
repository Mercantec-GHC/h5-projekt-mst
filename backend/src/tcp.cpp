#include "tcp.h"
#include <arpa/inet.h>
#include <cstdlib>
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

auto event::make_listener_event(int fd) -> Event*
{
    auto ev = (Event*)malloc(sizeof(Event));

    ev->variant = Listener;
    ev->data.listener_fd = fd;

    return ev;
}

auto event::make_connection_event(TcpConnection connection) -> Event*
{
    auto ev = (Event*)malloc(sizeof(Event));
    auto ptr = (TcpConnection*)std::malloc(sizeof(TcpConnection));
    *ptr = connection;

    ev->variant = Connection;
    ev->data.connection = ptr;

    return ev;
}

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

    auto epoll_fd = ::epoll_create1(0);

    auto context = event::make_listener_event(socket_fd);

    auto event = epoll_event { .events = EPOLLIN, .data = { .ptr = context } };
    if (::epoll_ctl(epoll_fd, EPOLL_CTL_ADD, socket_fd, &event) < 0) {
        return std::unexpected(errno_shim("could not connect to epoll"));
    }

    return TcpListener(epoll_fd, address);
}

auto TcpListener::start() -> Result<void>
{
    epoll_event events[128] = { };
    while (true) {

        auto events_len = ::epoll_wait(this->epoll_fd, events, 128, -1);
        if (events_len < 0) {
            return std::unexpected(errno_shim("could not poll"));
        }
        for (int i = 0; i < events_len; ++i) {
            auto event = (event::Event*)events[i].data.ptr;

            switch (event->variant) {
                case event::Listener: {
                    socklen_t size = sizeof(address);
                    int client = ::accept(
                        events[0].data.fd, (struct sockaddr*)&address, &size);
                    if (client < 0) {
                        return std::unexpected(errno_shim("could not accept"));
                    }

                    auto context = event::make_connection_event(
                        TcpConnection(*this, client));

                    auto poll_event = epoll_event { .events = EPOLLIN,
                        .data = { .ptr = context } };

                    if (::epoll_ctl(
                            epoll_fd, EPOLL_CTL_ADD, client, &poll_event)
                        < 0) {
                        return std::unexpected(
                            errno_shim("could not add connection to epoll"));
                    }

                    break;
                }
                case event::Connection: {
                    event->data.connection.wake();
                    break;
                }
            }
        }
    }
}

}
