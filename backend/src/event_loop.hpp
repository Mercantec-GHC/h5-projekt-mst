#pragma once
#include "errno_shim.hpp"
#include "result.hpp"
#include <sys/epoll.h>
namespace mst::event {
enum Variant { Server, Client };
struct Event {
    Variant variant;
    int fd;
    void* data;
};

template <typename Data> auto make_event(Variant variant, Data data) -> Event*
{
    auto event = (Event*)malloc(sizeof(Event));
    auto ptr = (Data*)std::malloc(sizeof(Data));
    *ptr = data;

    event->variant = variant;
    event->data = ptr;

    return event;
}

class Manager {
public:
    auto start() -> Result<void>;
    template <typename Data>
    auto register_event(Variant variant, Data data, int fd) -> Result<void>
    {
        auto poll_event = epoll_event { .events = EPOLLIN,
            .data = { .ptr = make_event(variant, data) } };

        if (::epoll_ctl(this->epoll_fd, EPOLL_CTL_ADD, fd, &poll_event) < 0) {
            return std::unexpected(
                errno_shim("could not add listener to epoll"));
        }
        return { };
    }
    static auto create() -> Result<Manager>;

private:
    int epoll_fd;
    Manager(int epoll_fd)
        : epoll_fd(epoll_fd) { };
};
}
