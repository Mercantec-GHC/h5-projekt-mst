#include "event_loop.hpp"
#include "json.hpp"
#include "mqtt.hpp"
#include "server.hpp"
#include <chrono>
#include <cstdio>
#include <iostream>
#include <print>
#include <span>
#include <stdexcept>
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

#ifndef BACKEND_MQTT_HOST
#define BACKEND_MQTT_HOST "mosquitto"
#endif

int main(void)
{

    auto mqtt_client = mst::mqtt::Client(
        BACKEND_MQTT_HOST, BACKEND_MQTT_PORT, "test", "1234");

    mqtt_client.subscribe("/skateboard/update", [&](std::string_view text) {
        //
        std::println("Skateboard: {}", text);
        auto result = mst::json::parse(text);

        if (!result) {
            std::println(stderr,
                "error: {} at {}",
                result.error().message,
                result.error().loc.idx);
            return;
        }
        try {
            auto parsed = std::move(result.value());
            std::println(".rotation = {}",
                parsed->query(".rotation").value()->get_f64());
        } catch (std::runtime_error& ex) {
            std::println(stderr, "exception: {}", ex.what());
        }
    });

    auto mqtt_thread = std::thread([&]() {
        try {
            mqtt_client.run();
        } catch (mst::mqtt::Error& ex) {
            std::println(stderr, "MQTT Client failed: {}", ex.what());
            std::abort();
        }
    });

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

    mqtt_thread.join();
    return 0;
}
