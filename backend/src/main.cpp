#include "json.hpp"
#include "mqtt.hpp"
#include "server.hpp"
#include <print>
#include <stdexcept>
#include <string_view>
#include <sys/select.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <thread>

#ifndef BACKEND_TCP_PORT
#define BACKEND_TCP_PORT 8888
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

    auto server = mst::server::Server(BACKEND_TCP_PORT);

    mqtt_client.subscribe("/skateboard/update", [&](std::string_view text) {
        std::println("Skateboard sent: {}", text);
        try {
            auto parsed = *mst::json::parse(text);
            auto angle = parsed->query(".rotation").value()->get_f64();
            server.notify_subscribers(angle);
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

    server.listen();

    mqtt_thread.join();
    return 0;
}
