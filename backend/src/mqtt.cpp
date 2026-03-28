#include "mqtt.hpp"
#include <cerrno>
#include <cstdint>
#include <cstring>
#include <format>
#include <iostream>
#include <mosquitto.h>
#include <mutex>
#include <string>
#include <string_view>
#include <thread>

namespace mst::mqtt {

static auto static_mx = std::mutex();
static auto use_count = 0;
static bool mosquitto_initialized = false;

static void lib_init()
{
    auto lock = std::lock_guard(static_mx);

    if (mosquitto_initialized)
        return;

    int major;
    int minor;
    int revision;
    mosquitto_lib_version(&major, &minor, &revision);
    std::cout << std::format(
        "[MQTT] Initializing mosquitto {}.{}.{}\n", major, minor, revision);

    if (mosquitto_lib_init() != MOSQ_ERR_SUCCESS)
        throw Error("failed to initialize");

    mosquitto_initialized = true;
    use_count += 1;
}

static void lib_deinit()
{
    auto lock = std::lock_guard(static_mx);

    use_count -= 1;

    if (!mosquitto_initialized || use_count < 0)
        return;

    std::cout << std::format("[MQTT] Deinitializing mosquitto\n");

    mosquitto_lib_cleanup();
    mosquitto_initialized = false;
}

namespace callbacks {

    static void connect(struct mosquitto* mosq, void* obj, int rc)
    {
        (void)mosq;

        auto client = static_cast<Client*>(obj);
        client->cb_connect(rc);
    }
    static void disconnect(struct mosquitto* mosq, void* obj, int rc)
    {
        (void)mosq;
        (void)rc;

        auto client = static_cast<Client*>(obj);
        client->cb_disconnect();
    }
    static void publish(struct mosquitto* mosq, void* obj, int mid)
    {
        (void)mosq;
        (void)mid;

        auto client = static_cast<Client*>(obj);
        client->cb_publish();
    }
    static void message(struct mosquitto* mosq,
        void* obj,
        const struct mosquitto_message* message)
    {
        (void)mosq;

        auto client = static_cast<Client*>(obj);
        client->cb_message(
            message->topic, message->payload, (size_t)message->payloadlen);
    }
    static void subscribe(struct mosquitto* mosq,
        void* obj,
        int mid,
        int qos_count,
        const int* granted_qos)
    {
        (void)mosq;
        (void)mid;
        (void)qos_count;
        (void)granted_qos;

        auto client = static_cast<Client*>(obj);
        client->cb_subscribe();
    }
    static void unsubscribe(struct mosquitto* mosq, void* obj, int mid)
    {
        (void)mosq;
        (void)mid;

        auto client = static_cast<Client*>(obj);
        client->cb_unsubscribe();
    }

}

Client::Client(const std::string& hostname,
    uint16_t port,
    const std::string& username,
    const std::string& password)
    : m_inst(nullptr)
{
    lib_init();
    m_inst = mosquitto_new(nullptr, true, this);
    if (!m_inst)
        throw Error(
            std::format("failed to create instance({})", strerror(errno)));

    auto inst = static_cast<struct mosquitto*>(m_inst);

    mosquitto_threaded_set(inst, true);

    if (auto status
        = mosquitto_username_pw_set(inst, username.c_str(), password.c_str());
        status != MOSQ_ERR_SUCCESS) {

        throw Error(std::format("could not set username and password ({})",
            mosquitto_strerror(status)));
    }

    mosquitto_connect_callback_set(inst, callbacks::connect);
    mosquitto_disconnect_callback_set(inst, callbacks::disconnect);
    mosquitto_publish_callback_set(inst, callbacks::publish);
    mosquitto_message_callback_set(inst, callbacks::message);
    mosquitto_subscribe_callback_set(inst, callbacks::subscribe);
    mosquitto_unsubscribe_callback_set(inst, callbacks::unsubscribe);

    if (auto status = mosquitto_connect(inst, hostname.c_str(), port, 5);
        status != MOSQ_ERR_SUCCESS) {
        if (status == MOSQ_ERR_ERRNO) {
            throw Error(std::format("could not connect ({})", strerror(errno)));
        } else {
            throw Error(std::format(
                "could not connect ({})", mosquitto_strerror(status)));
        }
    }
}

Client::~Client()
{
    auto inst = static_cast<struct mosquitto*>(m_inst);
    mosquitto_disconnect(inst);
    mosquitto_destroy(inst);
    lib_deinit();
}

void Client::subscribe(
    std::string topic, std::function<void(std::string_view)> func)
{
    auto inst = static_cast<struct mosquitto*>(m_inst);

    m_subscriptions.emplace_back(topic, std::move(func));

    if (auto status = mosquitto_subscribe(inst, NULL, topic.c_str(), 0);
        status != MOSQ_ERR_SUCCESS) {

        throw Error(std::format(
            "could not subscribe ({})", mosquitto_strerror(status)));
    }
}

void Client::run()
{

    auto inst = static_cast<struct mosquitto*>(m_inst);

    if (auto status = mosquitto_loop_forever(inst, -1, 1);
        status != MOSQ_ERR_SUCCESS) {

        if (status == MOSQ_ERR_ERRNO) {
            throw Error(std::format("lost connection ({})", strerror(errno)));
        } else {
            throw Error(std::format(
                "lost connection ({})", mosquitto_strerror(status)));
        }
    }
}

void Client::cb_connect(int rc)
{
    if (rc != 0) {
        throw Error(std::format(
            "client could not connect ({})", mosquitto_reason_string(rc)));
    }

    std::cout << std::format("[MQTT] Client connected\n");
}

void Client::cb_disconnect()
{
    std::cout << std::format("[MQTT] Client disconnected\n");
}

void Client::cb_publish()
{
    std::cout << std::format("[MQTT] Message published\n");
}

void Client::cb_message(std::string_view topic, const void* data, size_t size)
{
    std::cout << std::format("[MQTT] Message received\n");

    auto text = std::string_view(static_cast<const char*>(data), size);

    for (auto sub : m_subscriptions) {
        if (sub.topic == topic) {
            sub.func(text);
        }
    }
}

void Client::cb_subscribe()
{
    std::cout << std::format("[MQTT] Client subscribed\n");
}

void Client::cb_unsubscribe()
{
    std::cout << std::format("[MQTT] Client unsubscribed\n");
}

void Client::publish_raw(
    const std::string& topic, const void* data, size_t size)
{
    auto inst = static_cast<struct mosquitto*>(m_inst);

    if (auto status = mosquitto_publish(
            inst, nullptr, topic.c_str(), (int)size, data, 0, false);
        status != MOSQ_ERR_SUCCESS) {

        throw Error(
            std::format("could not publish ({})", mosquitto_strerror(status)));
    }
}
}
