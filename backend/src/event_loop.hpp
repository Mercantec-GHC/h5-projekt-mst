#pragma once
#include "errno_shim.hpp"
#include "result.hpp"
#include "server.hpp"
#include <memory>
#include <sys/epoll.h>
#include <variant>
#include <vector>

namespace mst {
class Server;
class Client;
namespace event {
    struct Event {
        std::variant<std::unique_ptr<mst::Server>, std::unique_ptr<mst::Client>>
            data;
    };

    template <typename Data>
    auto make_event(Data&& data) -> std::unique_ptr<Event>
    {
        return std::make_unique<Event>(std::make_unique<Data>(std::move(data)));
    }

    class Manager {
    public:
        auto start() -> Result<void>;
        template <typename Data>
        auto register_event(Data&& data, int fd) -> Result<void>
        {

            auto event = make_event(std::move(data));
            this->events.emplace_back(std::move(event));
            auto poll_event = epoll_event { .events = EPOLLIN,
                .data = { .ptr = events.back().get() } };

            if (::epoll_ctl(this->epoll_fd, EPOLL_CTL_ADD, fd, &poll_event)
                < 0) {
                return std::unexpected(
                    errno_shim("could not add listener to epoll"));
            }
            return { };
        }
        static auto create() -> Result<Manager>;

    private:
        int epoll_fd;
        std::vector<std::unique_ptr<Event>> events;
        Manager(int epoll_fd)
            : epoll_fd(epoll_fd) { };
    };
}
}
