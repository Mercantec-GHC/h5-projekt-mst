#pragma once
#include <expected>
#include <string>

template <typename T> using Result = std::expected<T, std::string>;
