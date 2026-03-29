#pragma once

#include <cassert>
#include <cstdint>
#include <cstdio>
#include <expected>
#include <memory>
#include <ostream>
#include <string>
#include <string_view>
#include <unordered_map>
#include <variant>
#include <vector>

namespace mst::json {

struct Loc {
    size_t idx;
    int line;
    int col;
};

struct Error {
    Loc loc;
    std::string message;
};

template <typename V, typename E = Error> using Result = std::expected<V, E>;

enum class Type {
    Null = 1,
    False,
    True,
    I64,
    F64,
    String,
    Array,
    Object,
};

enum class WriteProfile {
    Minified,
    Pretty,
};

class Value {
public:
    using I64 = std::int64_t;
    using F64 = double;
    using String = std::string;
    using Array = std::vector<std::unique_ptr<Value>>;
    using Object = std::unordered_map<std::string, std::unique_ptr<Value>>;

    using Data = std::variant<std::monostate, I64, F64, String, Array, Object>;

    Value(const Value&) = delete;
    Value(Value&&) = delete;
    Value& operator=(const Value&) = delete;
    Value& operator=(Value&&) = delete;

    static auto make_null() -> std::unique_ptr<Value>
    {
        return std::make_unique<Value>(Type::Null);
    }
    static auto make_bool(bool value) -> std::unique_ptr<Value>
    {
        return std::make_unique<Value>(value ? Type::True : Type::False);
    }
    static auto make_i64(I64 value) -> std::unique_ptr<Value>
    {
        return std::make_unique<Value>(Type::I64, value);
    }
    static auto make_f64(F64 value) -> std::unique_ptr<Value>
    {
        return std::make_unique<Value>(Type::F64, value);
    }
    static auto make_string(std::string value) -> std::unique_ptr<Value>
    {
        return std::make_unique<Value>(Type::String, std::move(value));
    }
    static auto make_array() -> std::unique_ptr<Value>
    {
        return std::make_unique<Value>(Type::Array, Array());
    }
    static auto make_object() -> std::unique_ptr<Value>
    {
        return std::make_unique<Value>(Type::Object, Object());
    }

    explicit Value(Type type)
        : m_type(std::move(type))
        , m_data(std::monostate())
    {
    }

    explicit Value(Type type, Data data)
        : m_type(std::move(type))
        , m_data(std::move(data))
    {
    }

    inline auto type() const -> Type
    {
        return m_type;
    }

    inline auto is(Type type) const -> bool
    {
        return m_type == type;
    }

    inline auto get_bool() const -> bool
    {
        assert(is(Type::False) || is(Type::True));
        return is(Type::True);
    }

    inline auto get_i64() const -> I64
    {
        assert(is(Type::I64));
        return std::get<I64>(m_data);
    }

    inline auto get_f64() const -> F64
    {
        assert(is(Type::F64));
        return std::get<F64>(m_data);
    }

    inline auto get_string() & -> String&
    {
        assert(is(Type::String));
        return std::get<String>(m_data);
    }
    inline auto get_string() const& -> const String&
    {
        assert(is(Type::String));
        return std::get<String>(m_data);
    }
    inline auto get_underlying_array() & -> Array&
    {
        assert(is(Type::Array));
        return std::get<Array>(m_data);
    }
    inline auto get_underlying_array() const& -> const Array&
    {
        assert(is(Type::Array));
        return std::get<Array>(m_data);
    }
    inline auto get_underlying_object() & -> Object&
    {
        assert(is(Type::Object));
        return std::get<Object>(m_data);
    }
    inline auto get_underlying_object() const& -> const Object&
    {
        assert(is(Type::Object));
        return std::get<Object>(m_data);
    }

    inline auto operator[](size_t idx) & -> Value&
    {
        return *get_underlying_array().at(idx);
    }
    inline auto operator[](size_t idx) const& -> const Value&
    {
        return *get_underlying_array().at(idx);
    }
    inline auto operator[](const std::string& key) -> Value&
    {
        return *get_underlying_object().at(std::string(key));
    }
    inline auto operator[](const std::string& key) const -> const Value&
    {
        return *get_underlying_object().at(std::string(key));
    }

    inline auto size() const -> size_t
    {
        assert(is(Type::Array) || is(Type::Object));
        if (is(Type::Array))
            return get_underlying_array().size();
        return get_underlying_object().size();
    }
    inline auto has(size_t idx) const -> bool
    {
        return idx < get_underlying_array().size();
    }
    inline auto has(const std::string& key) const -> bool
    {
        return get_underlying_object().contains(key);
    }

    inline auto clone() const -> std::unique_ptr<Value>
    {
        return std::make_unique<Value>(m_type, Data(m_data));
    }

    inline auto push(std::unique_ptr<Value> value)
    {
        get_underlying_array().push_back(std::move(value));
    }
    inline auto push(Value&& value)
    {
        get_underlying_array().push_back(
            std::make_unique<Value>(value.m_type, std::move(value.m_data)));
    }
    [[deprecated("move or use explicit .clone() instead")]]
    inline auto push(Value& value)
    {
        get_underlying_array().push_back(
            std::make_unique<Value>(value.m_type, std::move(value.m_data)));
    }

    inline auto set(const std::string& key, std::unique_ptr<Value> value)
    {
        get_underlying_object()[key] = std::move(value);
    }
    inline auto set(const std::string& key, Value&& value)
    {
        get_underlying_object()[key]
            = std::make_unique<Value>(value.m_type, std::move(value.m_data));
    }
    [[deprecated("use explicit .clone() instead")]]
    inline auto set(const std::string& key, Value& value)
    {
        get_underlying_object()[key]
            = std::make_unique<Value>(value.m_type, std::move(value.m_data));
    }

    inline void set_null()
    {
        m_type = Type::Null;
        m_data = std::monostate();
    }
    inline void set_bool(bool value)
    {
        m_type = value ? Type::True : Type::False;
        m_data = std::monostate();
    }
    inline void set_i64(I64 value)
    {
        m_type = Type::I64;
        m_data = value;
    }
    inline void set_f64(F64 value)
    {
        m_type = Type::F64;
        m_data = value;
    }
    inline void set_string(String value)
    {
        m_type = Type::String;
        m_data = std::move(value);
    }
    inline void set_array()
    {
        m_type = Type::Array;
        m_data = Array();
    }
    inline void set_object()
    {
        m_type = Type::Object;
        m_data = Object();
    }

    auto query(std::string_view path) & -> Result<Value*, std::string>;
    auto query(
        std::string_view path) const& -> Result<const Value*, std::string>;

    auto write(
        std::FILE* file, WriteProfile profile = WriteProfile::Minified) const;
    auto write(std::ostream& stream,
        WriteProfile profile = WriteProfile::Minified) const;
    auto to_string(WriteProfile profile = WriteProfile::Minified)
        -> std::string;

private:
    Type m_type;
    Data m_data;
};

auto parse(std::string_view text) -> Result<std::unique_ptr<Value>>;

}
