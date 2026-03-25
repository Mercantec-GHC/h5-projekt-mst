#include "server.hpp"
#include "errno_shim.hpp"
#include "event_loop.hpp"
#include <print>
#include <sys/epoll.h>

namespace mst {

auto Client::wake() -> Result<void>
{
    uint8_t buffer[128] = { };
    auto x = this->connection.read(buffer, 128);
    if (!x) {
        return std::unexpected(x.error());
    }
    auto bytes_read = x.value();
    for (size_t i = 0; i < bytes_read; ++i) {
        std::println("{:c}", buffer[i]);
    }

    return { };
}

auto Server::bind(mst::event::Manager& mgr, const std::string& host,
    uint16_t port) -> Result<void>
{
    auto x = TcpListener::bind(host, port);
    if (!x) {
        return std::unexpected(x.error());
    }
    auto listener = x.value();
    auto context = event::make_event(event::Server, Server(listener));
    auto poll_event
        = epoll_event { .events = EPOLLIN, .data = { .ptr = context } };
    if (::epoll_ctl(mgr.epoll_fd, EPOLL_CTL_ADD, listener.fd, &poll_event)
        < 0) {
        return std::unexpected(errno_shim("could not add listener to epoll"));
    }
    return { };
}

auto Server::wake(event::Manager& mgr) -> Result<void>
{
    auto x = this->listener.accept();
    auto connection = x.value();

    auto context
        = event::make_event(event::Client, mst::Client(*this, connection));

    auto poll_event
        = epoll_event { .events = EPOLLIN, .data = { .ptr = context } };

    if (::epoll_ctl(mgr.epoll_fd, EPOLL_CTL_ADD, connection.fd, &poll_event)
        < 0) {
        return std::unexpected(
            mst::errno_shim("could not add connection to epoll"));
    }
    return { };
}

}
