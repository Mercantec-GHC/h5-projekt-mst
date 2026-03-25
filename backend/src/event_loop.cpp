#include "event_loop.hpp"
#include "errno_shim.hpp"
#include "server.hpp"
#include <print>
#include <sys/epoll.h>

namespace mst::event {

auto Manager::start() -> Result<void>
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
                case event::Server: {
                    auto ptr = (mst::Server*)event->data;
                    auto res = ptr->wake(*this);
                    if (!res) {
                        return std::unexpected(res.error());
                    }

                    break;
                }
                case event::Client: {
                    std::println("client call scheduled");
                    auto ptr = (mst::Client*)event->data;
                    auto res = ptr->wake();
                    if (!res) {
                        return std::unexpected(res.error());
                    }
                    break;
                }
            }
        }
    }
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
