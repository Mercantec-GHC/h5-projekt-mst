#include "event_loop.hpp"
#include "json.hpp"
#include "mqtt.hpp"
#include "server.hpp"
#include <chrono>
#include <cstdio>
#include <iostream>
#include <print>
#include <span>
#include <string_view>
#include <sys/select.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <thread>

#ifdef BACKEND_TCP_PORT
#define PORT BACKEND_TCP_PORT
#else
#define PORT 8888
#endif

#ifndef BACKEND_MQTT_PORT
#define BACKEND_MQTT_PORT 1883
#endif

int main(void)
{
    auto mqtt_client
        = mst::mqtt::Client("10.133.51.127", BACKEND_MQTT_PORT, "test", "1234");

    mqtt_client.subscribe("/skateboard/update", [&](std::string_view text) {
        //
        std::println("Skateboard: {}", text);
        // auto parsed = mst::json::parse(text).value();

        // std::println(".acceleration[0]",
        //     parsed->query(".acceleration[0]").value()->get_i64());
    });

    auto mqtt_thread = std::thread([&]() {
        try {
            mqtt_client.run();
        } catch (mst::mqtt::Error& ex) {
            std::println(stderr, "MQTT Client failed: {}", ex.what());
            std::abort();
        }
    });

    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    mqtt_client.publish("/", "published from c++");

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
