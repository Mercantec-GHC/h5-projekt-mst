#pragma once
#include "result.hpp"
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
    int epoll_fd;
    static auto create() -> Result<Manager>;

private:
    Manager(int epoll_fd)
        : epoll_fd(epoll_fd) { };
};
}
