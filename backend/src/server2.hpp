#pragma once

#include <memory>
#include <stdexcept>

namespace mst::server2 {

struct Error : public std::runtime_error {
    using std::runtime_error::runtime_error;
};

class Server {
public:
    explicit Server();
    ~Server();

    void listen();

private:
    struct State;

    void create_connection();
    void handle_request(size_t i);

    int m_listener_fd { };
    std::unique_ptr<State> m_state { nullptr };
};

}
