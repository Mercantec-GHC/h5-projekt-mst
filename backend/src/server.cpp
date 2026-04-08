#include "server.hpp"
#include "event_loop.hpp"
#include <print>
#include <sys/epoll.h>

namespace mst {

auto Client::wake() -> Result<Client::lllll>
{
    uint8_t buffer[128] = { };
    auto x = this->connection.read(buffer, 128);
    if (!x) {
        return std::unexpected(x.error());
    }
    auto bytes_read = x.value();
    if (bytes_read == 0) {
        return { Client::lllll::Disconnect };
    }
    for (size_t i = 0; i < bytes_read; ++i) {
        std::println("{:c}", buffer[i]);
    }

    return { Client::lllll::Ok };
}

auto Server::bind(
    mst::event::Manager& mgr, const std::string& host, uint16_t port)
    -> Result<void>
{
    auto x = TcpListener::bind(host, port);
    if (!x) {
        return std::unexpected(x.error());
    }
    auto listener = x.value();
    auto res
        = mgr.register_event(event::make_event(Server(listener)), listener.fd);
    if (!res) {
        return std::unexpected(res.error());
    }
    return { };
}

auto Server::wake(event::Manager& mgr) -> Result<void>
{
    auto x = this->listener.accept();
    auto connection = x.value();

    auto res = mgr.register_event(
        event::make_event(mst::Client(*this, connection)), connection.fd);

    if (!res) {
        return std::unexpected(res.error());
    }
    return { };
}

}
