#pragma once

#include <cstddef>
#include <cstdint>
#include <functional>
#include <stdexcept>
#include <string>
#include <string_view>

namespace mst::mqtt {

struct Error : public std::runtime_error {
    using std::runtime_error::runtime_error;
};

struct Subscription {
    std::string topic;
    std::function<void(std::string_view)> func;
};

class Client {
public:
    explicit Client(const std::string& hostname,
        uint16_t port,
        const std::string& username,
        const std::string& password);

    ~Client();

    void publish(const std::string& topic, std::string_view text)
    {
        publish_raw(topic, text.data(), text.size());
    }

    void subscribe(
        std::string topic, std::function<void(std::string_view)> func);

    void run();

    void cb_connect(int rc);
    void cb_disconnect();
    void cb_publish();
    void cb_message(std::string_view topic, const void* data, size_t size);
    void cb_subscribe();
    void cb_unsubscribe();

private:
    void publish_raw(const std::string& topic, const void* data, size_t size);

    void* m_inst;
    std::vector<Subscription> m_subscriptions;
};

};
