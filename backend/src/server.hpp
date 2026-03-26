#pragma once

#include "event_loop.hpp"
#include "tcp.hpp"

namespace mst {

namespace event {
    class Manager;
}

class Server {
public:
    auto wake(mst::event::Manager& mgr) -> Result<void>;
    static auto bind(
        mst::event::Manager& mgr, const std::string& host, uint16_t port)
        -> Result<void>;

private:
    TcpListener listener;

    Server(TcpListener listener)
        : listener(listener)
    {
    }
};

class Client {
public:
    auto wake() -> Result<void>;

    Client(Server&, TcpConnection connection)
        : connection(connection)
    {
    }

private:
    TcpConnection connection;
};

}
