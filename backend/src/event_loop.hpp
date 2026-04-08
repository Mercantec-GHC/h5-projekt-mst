#pragma once
#include "errno_shim.hpp"
#include "result.hpp"
#include "server.hpp"
#include <algorithm>
#include <memory>
#include <sys/epoll.h>
#include <utility>
#include <variant>
#include <vector>

namespace mst {
class Server;
class Client;
namespace event {
    enum class EventKind {
        Server,
        Client,
    };
    class Event {
    public:
        Event(std::variant<std::unique_ptr<mst::Server>,
            std::unique_ptr<mst::Client>> data)
            : data(std::move(data))
            , kind(data.index() == 0 ? EventKind::Server : EventKind::Client)
        {
        }
        auto fd() -> int;

        std::variant<std::unique_ptr<mst::Server>, std::unique_ptr<mst::Client>>
            data;
        EventKind kind;
    };

    template <typename Data>
    static auto make_event(Data&& data) -> std::unique_ptr<Event>
    {
        return std::make_unique<Event>(std::make_unique<Data>(std::move(data)));
    }

    class Manager {
    public:
        auto start() -> Result<void>;
        auto register_event(std::unique_ptr<Event> event, int fd)
            -> Result<void>;
        auto deregister_event(int fd) -> Result<void>;
        static auto create() -> Result<Manager>;

    private:
        int epoll_fd;
        std::vector<std::unique_ptr<Event>> events;
        Manager(int epoll_fd)
            : epoll_fd(epoll_fd) { };
    };
}
}
