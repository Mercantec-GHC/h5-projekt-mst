#pragma once

#include <cstdlib>
#include <format>
#include <iostream>

#define ASSERT_EQ(EXPR, VALUE)                                                 \
    do {                                                                       \
        if ((EXPR) != (VALUE)) {                                               \
            std::cerr << std::format(                                          \
                "ASSERTION FAILED: Expected '{}' ({}) to be '{}' ({})\n",      \
                #EXPR,                                                         \
                (EXPR),                                                        \
                #VALUE,                                                        \
                (VALUE));                                                      \
            std::exit(1);                                                      \
        }                                                                      \
    } while (false);
