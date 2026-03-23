#include "http.h"

namespace mst {
auto HttpServer::start() -> Result<void>
{
    while (1) {
        auto res = this->listener.accept();
        if (res) { }
    }
};

auto HttpServer::bind(const std::string& host, uint16_t port)
    -> Result<HttpServer>
{
    auto x = TcpListener::bind(host, port);

    if (!x) {
        return std::unexpected(std::move(x.error()));
    }

    return HttpServer(*x);
}
}
