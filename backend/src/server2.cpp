#include "server2.hpp"
#include <algorithm>
#include <cerrno>
#include <cstdint>
#include <cstring>
#include <format>
#include <iterator>
#include <netdb.h>
#include <poll.h>
#include <print>
#include <sys/poll.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>
#include <vector>

namespace {
using namespace mst::server2;

auto get_listener_socket() -> int
{
    int listener; // Listening socket descriptor
    int status;

    struct addrinfo hints = { };
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_flags = AI_PASSIVE;

    struct addrinfo* addr;

    if ((status = ::getaddrinfo(NULL, "8881", &hints, &addr)) != 0)
        throw Error(std::format("getaddrinfo ({})", ::gai_strerror(status)));

    struct addrinfo* p;
    for (p = addr; p != NULL; p = p->ai_next) {
        listener = ::socket(p->ai_family, p->ai_socktype, p->ai_protocol);
        if (listener < 0) {
            continue;
        }

        int reuseaddr_opt = 1;
        ::setsockopt(
            listener, SOL_SOCKET, SO_REUSEADDR, &reuseaddr_opt, sizeof(int));

        if (::bind(listener, p->ai_addr, p->ai_addrlen) < 0) {
            ::close(listener);
            continue;
        }

        break;
    }

    if (p == NULL)
        throw Error("didn't get bound");

    ::freeaddrinfo(addr);

    if (::listen(listener, 10) == -1)
        throw Error(std::format("could not listen ({})", strerror(errno)));

    return listener;
}

}

namespace mst::server2 {

struct Server::State {
    std::vector<::pollfd> pollfds;
    std::vector<::pollfd> queued_insertions;
    std::vector<std::size_t> queued_deletions;
};

Server::Server()
    : m_state(std::make_unique<State>())
{
}
Server::~Server() = default;

void Server::listen()
{
    m_listener_fd = get_listener_socket();
    m_state->pollfds.push_back(::pollfd {
        .fd = m_listener_fd,
        .events = POLLIN,
        .revents = { },
    });

    std::println("[mst::server2] listening for connections");

    while (true) {
        int poll_count
            = ::poll(m_state->pollfds.data(), m_state->pollfds.size(), -1);

        if (poll_count == -1)
            throw Error(std::format("poll (%s)", strerror(errno)));

        for (size_t i = 0; i < m_state->pollfds.size(); ++i) {
            auto& fd = m_state->pollfds[i];
            if (!(fd.revents & (POLLIN | POLLHUP)))
                continue;

            if (fd.fd == m_listener_fd) {
                create_connection();
            } else {
                try {
                    handle_request(i);
                } catch (Error& ex) {
                    std::println(stderr,
                        "[mst::server2] exception in handler for client {}: {}",
                        fd.fd,
                        ex.what());
                    m_state->queued_deletions.push_back(i);
                }
            }
        }

        auto& fds = m_state->pollfds;
        auto& deletions = m_state->queued_deletions;

        std::reverse(deletions.begin(), deletions.end());
        for (auto idx : deletions) {
            fds.erase(std::next(fds.begin(), static_cast<long>(idx)));
        }
        m_state->queued_deletions.clear();

        for (auto& fd : m_state->queued_insertions) {
            m_state->pollfds.push_back(fd);
        }
        m_state->queued_insertions.clear();
    }
}

void Server::create_connection()
{
    struct sockaddr_storage remoteaddr;
    socklen_t addrlen = sizeof remoteaddr;

    int client_fd
        = ::accept(m_listener_fd, (struct sockaddr*)&remoteaddr, &addrlen);

    if (client_fd == -1)
        throw Error(std::format("format ({})", strerror(errno)));

    std::println("[mst::server2] client {} connected", client_fd);

    m_state->queued_insertions.push_back(::pollfd {
        .fd = client_fd,
        .events = POLLIN,
        .revents = { },
    });
}

void Server::handle_request(size_t i)
{
    auto& client_fd = m_state->pollfds[i].fd;
    auto buffer = std::vector<char>(512);

    ssize_t byte_count = ::recv(client_fd, buffer.data(), buffer.size(), 0);

    if (byte_count <= 0)
        throw Error(std::format("recv: {}", strerror(errno)));

    if (byte_count == 0) {
        std::println("[mst::server2] client {} disconnected", client_fd);

        m_state->queued_deletions.push_back(i);
        return;
    }

    std::println("[mst::server2] received: {:s}", buffer);
}

}
