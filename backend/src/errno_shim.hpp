#pragma once
#include <cerrno>
#include <cstring>
#include <format>
#include <string_view>

namespace mst {
auto inline errno_shim(std::string_view message) -> std::string
{
    return std::format("{} ({})", message, strerror(errno));
}
}
