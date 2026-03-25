#include "event_loop.hpp"
#include "server.hpp"
#include <print>
#include <sys/select.h>
#include <sys/socket.h>
#include <sys/types.h>

#define PORT 8888

int main(void)
{
    auto mgr = mst::event::Manager::create().value();
    auto x = mst::Server::bind(mgr, "0.0.0.0", PORT);
    if (!x) {
        std::println("{}", x.error());
        return 1;
    }
    std::println("starting");
    {
        auto x = mgr.start();
    }
    return 0;
}
