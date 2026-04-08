#include "event_loop.hpp"
#include "errno_shim.hpp"
#include "server.hpp"
#include <algorithm>
#include <print>
#include <stdexcept>
#include <sys/epoll.h>
#include <utility>

#define CHECK(EXPR)                                                            \
    do {                                                                       \
        if (!(EXPR).has_value()) [[unlikely]] {                                \
            return std::unexpected((EXPR).error());                            \
        }                                                                      \
    } while (false)

namespace mst::event {
auto Event::fd() -> int
{
    switch (this->kind) {
        case mst::event::EventKind::Server: {
            auto& ref = std::get<std::unique_ptr<mst::Server>>(this->data);
            return ref->fd();
            break;
        }
        case mst::event::EventKind::Client: {
            auto& ref = std::get<std::unique_ptr<mst::Client>>(this->data);
            return ref->fd();
        }
        default:
            std::unreachable();
    }
}
auto Manager::start() -> Result<void>
{
    epoll_event events[128] = { };
    while (true) {
        auto events_len = ::epoll_wait(this->epoll_fd, events, 128, -1);
        if (events_len < 0) {
            return std::unexpected(errno_shim("could not poll"));
        }
        for (int i = 0; i < events_len; ++i) {
            auto event = (mst::event::Event*)events[i].data.ptr;
            switch (event->kind) {
                case mst::event::EventKind::Server: {
                    auto& ref
                        = std::get<std::unique_ptr<mst::Server>>(event->data);
                    auto res = ref->wake(*this);
                    CHECK(res);
                    break;
                }
                case mst::event::EventKind::Client: {
                    auto& ref
                        = std::get<std::unique_ptr<mst::Client>>(event->data);
                    auto res = ref->wake();
                    CHECK(res);
                    auto do_now = res.value();
                    if (do_now == Client::lllll::Disconnect) {
                        CHECK(this->deregister_event(ref->fd()));
                    }
                    break;
                }
                default:
                    std::unreachable();
            }
        }
    }
}

auto Manager::register_event(std::unique_ptr<Event> event, int fd)
    -> Result<void>
{
    this->events.emplace_back(std::move(event));
    auto poll_event = epoll_event { .events = EPOLLIN,
        .data = { .ptr = events.back().get() } };

    if (::epoll_ctl(this->epoll_fd, EPOLL_CTL_ADD, fd, &poll_event) < 0) {
        return std::unexpected(errno_shim("could not add listener to epoll"));
    }
    return { };
}

auto Manager::deregister_event(int fd) -> Result<void>
{
    auto poll_event = epoll_event { .events = EPOLLIN, .data = { } };
    if (::epoll_ctl(this->epoll_fd, EPOLL_CTL_DEL, fd, &poll_event) < 0) {
        return std::unexpected(
            errno_shim("could not remove listener to epoll"));
    }
    size_t idx = this->events.size();
    for (size_t i = 0; i < this->events.size(); ++i) {
        if (this->events[i]->fd() == fd) {
            idx = i;
        }
    }

    if (idx == this->events.size()) {
        throw std::runtime_error("contract broken");
    }

    this->events.erase(std::find_if(this->events.begin(),
        this->events.end(),
        [&](auto& e) { return e->fd() == fd; }));

    return { };
}

auto Manager::create() -> Result<Manager>
{
    auto epoll_fd = ::epoll_create1(0);
    if (epoll_fd < 0) {
        std::unexpected(mst::errno_shim("could not create epoll"));
    }

    return Manager(epoll_fd);
}
}
