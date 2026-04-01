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
    auto fd() -> int
    {
        return this->listener.fd;
    };

private:
    TcpListener listener;

    Server(TcpListener listener)
        : listener(listener)
    {
    }
};

class Client {

public:
    enum class lllll {
        Ok,
        Disconnect,
    };
    auto wake() -> Result<lllll>;
    auto fd() -> int
    {
        return this->connection.fd;
    };

    Client(Server&, TcpConnection connection)
        : connection(connection)
    {
    }

private:
    TcpConnection connection;
};

}
