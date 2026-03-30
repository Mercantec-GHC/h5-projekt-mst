#include "../src/json.hpp"
#include "assertions.h"

namespace json = mst::json;

int main()
{
    ASSERT_EQ(**json::parse("null"), *json::Value::make_null());
    ASSERT_EQ(**json::parse("false"), *json::Value::make_bool(false));
    ASSERT_EQ(**json::parse("true"), *json::Value::make_bool(true));
    ASSERT_EQ(**json::parse("0"), *json::Value::make_i64(0));
    ASSERT_EQ(**json::parse("123"), *json::Value::make_i64(123));
    ASSERT_EQ(**json::parse("3.14"), *json::Value::make_f64(3.14));
    ASSERT_EQ(**json::parse("\"hello world\""),
        *json::Value::make_string("hello world"));
}
