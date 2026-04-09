#pragma once

#include <cstdint>
#include <memory>
#include <stdexcept>

namespace mst::server {

struct Error : public std::runtime_error {
    using std::runtime_error::runtime_error;
};

class Server {
public:
    explicit Server(uint16_t port);
    ~Server();

    void listen();
    void notify_subscribers(double angle);

private:
    struct State;

    void create_connection();
    void handle_request(size_t i);

    uint16_t m_port;
    int m_listener_fd { };
    std::unique_ptr<State> m_state { nullptr };
};

}
