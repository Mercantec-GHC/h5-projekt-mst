#include "tcp.hpp"
#include <print>
#include <stdio.h>
#include <sys/select.h>
#include <sys/socket.h>
#include <sys/types.h>

#define PORT 8888

int main(void)
{
    auto x = mst::TcpListener::bind("0.0.0.0", PORT);
    if (!x) {
        std::println("{}", x.error());
        return 1;
    }
    std::println("starting");
    auto listener = x.value();
    {
        auto x = listener.loop();
    }
    return 0;
}
