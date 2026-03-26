#include "event_loop.hpp"
#include "errno_shim.hpp"
#include "server.hpp"
#include <sys/epoll.h>
#include <utility>
#include <variant>

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
            auto event = (mst::event::Event*)events[i].data.ptr;
            switch (event->data.index()) {
                case 0: {
                    auto& ref
                        = std::get<std::unique_ptr<mst::Server>>(event->data);
                    auto res = ref->wake(*this);
                    if (!res) {
                        return std::unexpected(res.error());
                    }
                    break;
                }
                case 1: {
                    auto& ref
                        = std::get<std::unique_ptr<mst::Client>>(event->data);
                    auto res = ref->wake();
                    if (!res) {
                        return std::unexpected(res.error());
                    }
                    break;
                }
                default:
                    std::unreachable();
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
