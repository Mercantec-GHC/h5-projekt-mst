#include "event_loop.hpp"
#include "mqtt.hpp"
#include "server.hpp"
#include <chrono>
#include <iostream>
#include <print>
#include <span>
#include <string_view>
#include <sys/select.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <thread>

#define PORT 8888

int main(void)
{
    auto client = mst::mqtt::Client("localhost", 1883, "test", "1234");

    client.subscribe("/", [&](std::string_view text) {
        std::cout << std::format("Received '{}'\n", text);
    });

    auto mqtt_thread = std::thread([&]() {
        try {
            client.run();
        } catch (mst::mqtt::Error& ex) {
            std::cerr << std::format("MQTT Client failed: {}", ex.what());
            std::abort();
        }
    });

    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    client.publish("/", "published from c++");

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
