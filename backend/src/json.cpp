#include "json.hpp"
#include <concepts>
#include <cstddef>
#include <cstring>
#include <format>
#include <memory>
#include <sstream>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>

namespace parse {

using namespace mst::json;

enum class TokTy {
    Eof,
    Null = std::to_underlying(Type::Null),
    False = std::to_underlying(Type::False),
    True = std::to_underlying(Type::True),
    String,
    Float,
    Int = '0',
    Comma = ',',
    Colon = ':',
    LBrace = '{',
    RBrace = '}',
    LBracket = '[',
    RBracket = '[',
};

struct Tok {
    TokTy ty;
    std::string_view text;
    Loc loc;
};

class Tokenizer {
public:
    Tokenizer(std::string_view text)
        : m_text(text)
        , m_len(text.size()) { };

    auto next() -> Result<Tok>
    {
        Loc loc = { m_idx, m_line, m_col };
        size_t* i = &m_idx;
        if (*i >= m_len) [[unlikely]] {
            return Tok { TokTy::Eof, std::string_view(nullptr, 0), loc };
        }
        bool matched = false;
        while (*i < m_len && std::strchr(" \t\r\n", m_text[*i]) != nullptr) {
            matched = true;
            step();
        }
        if (matched) {
            return next();
        }
        if (strchr(",:[]{}", m_text[*i]) != nullptr) {
            auto ty = (TokTy)m_text[*i];
            step();
            return tok(ty, loc);
        }
        while (*i < m_len && m_text[*i] >= 'a' && m_text[*i] <= 'z') {
            matched = true;
            step();
        }
        if (matched) {
            return make_ident_tok(loc, i);
        }
        if (m_text[*i] == '-' || (m_text[*i] >= '0' && m_text[*i] <= '9')) {
            return make_number_tok(loc, i);
        }
        if (m_text[*i] == '\"') {
            return make_string_tok(loc, i);
        }
        return std::unexpected(Error { loc, "illegal character" });
    }

private:
    void step()
    {
        if (m_idx >= m_len) [[unlikely]] {
            return;
        }
        if (m_text[m_idx] == '\n') {
            m_line += 1;
            m_col = 1;
        } else {
            m_col += 1;
        }
        m_idx += 1;
    }

    auto tok(TokTy ty, Loc loc) -> Tok
    {
        return { ty, std::string_view(&m_text[loc.idx], m_idx - loc.idx), loc };
    }

    auto make_ident_tok(Loc loc, size_t* i) -> Result<Tok>
    {
        char const* kws[] = { "null", "false", "true" };
        size_t const lens[] = { 4, 5, 4 };
        TokTy tys[] = { TokTy::Null, TokTy::False, TokTy::True };
        for (size_t j = 0; j < sizeof(kws) / sizeof(kws[0]); ++j) {
            size_t len = *i - loc.idx;
            if (lens[j] == len && strncmp(kws[j], &m_text[loc.idx], len) == 0) {
                return tok(tys[j], loc);
            }
        }
        return std::unexpected(Error { loc, "invalid identifier" });
    }

    auto make_number_tok(Loc loc, size_t* i) -> Tok
    {
        step();
        while (*i < m_len && m_text[*i] >= '0' && m_text[*i] <= '9') {
            step();
        }
        auto ty = TokTy::Int;
        if (*i < m_len && m_text[*i] == '.') {
            ty = TokTy::Float;
            step();
            while (*i < m_len && m_text[*i] >= '0' && m_text[*i] <= '9') {
                step();
            }
        }
        return tok(ty, loc);
    }

    auto make_string_tok(Loc loc, size_t* i) -> Result<Tok>
    {
        step();
        while (*i < m_len && m_text[*i] != '\"' && m_text[*i] != '\n') {
            if (m_text[*i] == '\\') {
                step();
                if (*i >= m_len)
                    break;
            }
            step();
        }
        if (*i < m_len && m_text[*i] == '\n') [[unlikely]] {
            return std::unexpected(Error { loc, "malformed string" });
        }
        if (*i >= m_len && m_text[*i] != '\"') [[unlikely]] {
            return std::unexpected(Error { loc, "malformed string" });
        }
        step();
        return tok(TokTy::String, loc);
    }

    std::string_view m_text;
    size_t m_len;
    size_t m_idx = 0;
    int m_line = 1;
    int m_col = 1;
};

#define CHECK(EXPR)                                                            \
    do {                                                                       \
        if (!(EXPR).has_value()) [[unlikely]] {                                \
            return std::unexpected((EXPR).error());                            \
        }                                                                      \
    } while (false)

auto literal_to_string(std::string_view text)
    -> Result<std::string, std::string>
{
    auto result = std::string();
    for (size_t i = 1; i < text.size() - 1; ++i) {
        if (text[i] == '\\') [[unlikely]] {
            i += 1;
            if (i >= text.size()) [[unlikely]] {
                return std::unexpected("malformed string");
            }
            switch (text[i]) {
                case 'b':
                    result += '\b';
                    break;
                case 'f':
                    result += '\f';
                    break;
                case 'n':
                    result += '\n';
                    break;
                case 'r':
                    result += '\r';
                    break;
                case 't':
                    result += '\t';
                    break;
                case 'u':
                    return std::unexpected("uXXXX in string not supported");
                default:
                    result += text[i];
            }
        } else {
            result += text[i];
        }
    }
    return result;
}

class Parser {
public:
    Parser(std::string_view text)
        : m_tokenizer(text)
    {
        auto result = step();
        result.value();
    }

    auto parse() -> Result<std::unique_ptr<Value>>
    {
        Loc loc = m_tok.loc;
        TokTy* ty = &m_tok.ty;

        if (*ty == TokTy::Null || *ty == TokTy::False || *ty == TokTy::True) {
            auto val = std::make_unique<Value>(static_cast<Type>(*ty));
            CHECK(step());
            return val;
        } else if (*ty == TokTy::Int) {
            int64_t value = std::strtol(m_tok.text.data(), nullptr, 10);
            auto val = std::make_unique<Value>(Type::I64, value);
            CHECK(step());
            return val;
        } else if (*ty == TokTy::Float) {
            double value = std::strtod(m_tok.text.data(), nullptr);
            auto val = std::make_unique<Value>(Type::F64, value);
            CHECK(step());
            return val;
        } else if (*ty == TokTy::String) {
            auto string_value = literal_to_string(m_tok.text);
            if (!string_value) {
                return std::unexpected(Error { loc, string_value.error() });
            }
            auto val = std::make_unique<Value>(Type::String, *string_value);
            CHECK(step());
            return val;
        } else if (std::to_underlying(*ty) == '[') {
            return parse_array(ty);
        } else if (std::to_underlying(*ty) == '{') {
            return parse_object(ty);
        } else {
            return std::unexpected(Error { loc, "expected expression" });
        }
    }

private:
    auto parse_array(TokTy* ty) -> Result<std::unique_ptr<Value>>
    {
        CHECK(step());
        auto values = Value::Array();
        bool tail = false;
        while (*ty != TokTy::Eof
            && ((!tail && std::to_underlying(*ty) != ']')
                || (tail && std::to_underlying(*ty) == ','))) {
            if (tail)
                CHECK(step());
            auto child = parse();
            CHECK(child);
            values.push_back(std::move(*child));
            tail = true;
        }
        if (*ty == TokTy::Eof || std::to_underlying(*ty) != ']') {
            return std::unexpected(Error { m_tok.loc, "expected ']'" });
        }
        CHECK(step());
        return std::make_unique<Value>(Type::Array, std::move(values));
    }

    auto parse_object(TokTy* ty) -> Result<std::unique_ptr<Value>>
    {
        CHECK(step());
        auto fields = Value::Object();
        bool tail = false;
        while (*ty != TokTy::Eof
            && ((!tail && std::to_underlying(*ty) != '}')
                || (tail && std::to_underlying(*ty) == ','))) {
            if (tail)
                CHECK(step());
            if (*ty != TokTy::String) {
                return std::unexpected(Error { m_tok.loc, "expected string" });
            }
            auto key_value = literal_to_string(m_tok.text);
            if (!key_value) {
                return std::unexpected(Error { m_tok.loc, key_value.error() });
            }
            CHECK(step());
            if (std::to_underlying(*ty) != ':') {
                return std::unexpected(Error { m_tok.loc, "expected ':'" });
            }
            CHECK(step());
            auto child = parse();
            CHECK(child);
            fields[*key_value] = std::move(*child);
            tail = true;
        }
        if (*ty == TokTy::Eof || std::to_underlying(*ty) != '}') {
            return std::unexpected(Error { m_tok.loc, "expected '}'" });
        }
        CHECK(step());
        return std::make_unique<Value>(Type::Object, std::move(fields));
        ;
    }

    auto step() -> Result<void>
    {
        auto result = m_tokenizer.next();
        CHECK(result);
        m_tok = *result;
        return { };
    }

    Tokenizer m_tokenizer;
    Tok m_tok = { };
};

}

namespace query {
using namespace mst::json;

enum class TokTy {
    Eof,
    String,
    Ident,
    Int = '0',
    Dot = '.',
    LBracket = '[',
    RBracket = ']',
};

struct Tok {
    TokTy ty;
    std::string_view text;
    size_t idx;
};

class Tokenizer {
public:
    Tokenizer(std::string_view text)
        : m_text(text) { };

    auto next() -> Result<Tok, std::string>
    {
        size_t idx = m_idx;
        size_t* i = &m_idx;
        if (*i >= m_text.size()) {
            return Tok { TokTy::Eof, std::string_view(nullptr, 0), idx };
        }
        bool matched = false;
        while (*i < m_text.size() && strchr(" \t\r\n", m_text[*i]) != nullptr) {
            matched = true;
            step();
        }
        if (matched) {
            return next();
        }
        if (strchr(".[]0", m_text[*i]) != nullptr) {
            auto ty = static_cast<TokTy>(m_text[*i]);
            step();
            return tok(ty, idx);
        }
        while (*i < m_text.size() && m_text[*i] >= 'a' && m_text[*i] <= 'z') {
            matched = true;
            step();
        }
        if (matched) {
            return tok(TokTy::Ident, idx);
        }
        if (m_text[*i] >= '1' && m_text[*i] <= '9') {
            return make_number_tok(idx, i);
        }
        if (m_text[*i] == '\"') {
            return make_string_tok(idx, i);
        }
        return std::unexpected("illegal character");
    }

private:
    auto make_number_tok(size_t idx, size_t* i) -> Tok
    {
        while (*i < m_text.size() && m_text[*i] >= '0' && m_text[*i] <= '9') {
            step();
        }
        return tok(TokTy::Int, idx);
    }

    auto make_string_tok(size_t idx, size_t* i) -> Result<Tok, std::string>
    {
        step();
        while (*i < m_text.size() && m_text[*i] != '\"') {
            if (m_text[*i] == '\\') {
                step();
                if (*i >= m_text.size())
                    break;
            }
            step();
        }
        if (*i >= m_text.size() && m_text[*i] != '\"') {
            return std::unexpected("malformed string");
        }
        step();
        return tok(TokTy::String, idx);
    }

    auto tok(TokTy ty, size_t idx) -> Tok
    {
        return { ty, std::string_view(&m_text[idx], m_idx - idx), idx };
    }

    void step()
    {
        if (m_idx >= m_text.size())
            return;
        m_idx += 1;
    }

    std::string_view m_text;
    size_t m_idx = 0;
};

enum class PathSegTy {
    Eof,
    IdentKey,
    StringKey,
    Idx,
};

struct PathSeg {
    PathSegTy ty;
    Tok tok;
};

class Parser {
public:
    Parser(std::string_view text)
        : m_tokenizer(text)
        , m_tok(m_tokenizer.next().value())
    {
    }

    auto next() -> Result<PathSeg, std::string>
    {
        auto ty = &m_tok.ty;

        if (*ty == TokTy::Eof) {
            return PathSeg { .ty = PathSegTy::Eof, .tok = m_tok };
        } else if (std::to_underlying(*ty) == '.') {
            CHECK(step());
            auto tok = m_tok;
            if (*ty == TokTy::Eof || *ty != TokTy::Ident) {
                return std::unexpected("expected identifier");
            }
            CHECK(step());
            return PathSeg { .ty = PathSegTy::IdentKey, .tok = tok };
        } else if (std::to_underlying(*ty) == '[') {
            CHECK(step());
            auto tok = m_tok;
            PathSegTy seg_ty;
            if (tok.ty == TokTy::Int) {
                seg_ty = PathSegTy::Idx;
            } else if (tok.ty == TokTy::String) {
                seg_ty = PathSegTy::StringKey;
            } else {
                return std::unexpected("expected string or integer");
            }
            CHECK(step());
            if (*ty == TokTy::Eof || std::to_underlying(*ty) != ']') {
                return std::unexpected("expected ']'");
            }
            CHECK(step());
            return PathSeg { .ty = seg_ty, .tok = tok };
        } else {
            return std::unexpected("expected expression");
        }
    }

private:
    auto step() -> Result<void, std::string>
    {
        auto result = m_tokenizer.next();
        CHECK(result);
        m_tok = *result;
        return { };
    }

    Tokenizer m_tokenizer;
    Tok m_tok = { };
};

template <typename ValueT>
    requires std::same_as<std::remove_cvref_t<ValueT>, Value>
auto resolve(Parser& parser, ValueT& node) -> Result<ValueT*, std::string>
{
    auto seg = parser.next();
    CHECK(seg);
    switch (seg->ty) {
        case PathSegTy::Eof:
            return &node;
        case PathSegTy::IdentKey: {
            if (!node.is(Type::Object))
                return std::unexpected("expected object");
            auto key = std::string(seg->tok.text);
            if (!node.has(key))
                return std::unexpected(
                    std::format("no field with key '{}' in object", key));
            auto& child = node[key];
            return resolve(parser, child);
        }
        case PathSegTy::StringKey: {
            if (!node.is(Type::Object))
                return std::unexpected("expected object");
            auto key = parse::literal_to_string(seg->tok.text);
            CHECK(key);
            if (!node.has(*key))
                return std::unexpected(
                    std::format("no field with key '{}' in object", *key));
            auto& child = node[*key];
            return resolve(parser, child);
        }
        case PathSegTy::Idx: {
            if (!node.is(Type::Array))
                return std::unexpected("expected array");
            auto idx = strtoull(seg->tok.text.data(), nullptr, 10);
            if (!node.has(idx))
                return std::unexpected(std::format(
                    "array index {} out of bounds {}", idx, node.size()));
            auto& child = node[idx];
            return resolve(parser, child);
        }
    }
    assert(false);
}

}

namespace stringify {
using namespace mst::json;

auto string_to_literal(std::string_view text) -> std::string
{
    auto result = std::string();
    result += '"';
    for (auto ch : text) {
        switch (ch) {
            case '\b':
                result += "\\b";
                break;
            case 'f':
                result += "\\f";
                break;
            case 'n':
                result += "\\n";
                break;
            case 'r':
                result += "\\r";
                break;
            case 't':
                result += "\\t";
                break;
            case '\\':
                result += "\\\\";
                break;
            default:
                break;
                result += ch;
        }
    }
    result += '"';
    return result;
}

template <typename Writer> void write_indent(Writer& writer, int depth)
{
    for (int i = 0; i < depth; ++i) {
        writer << "  ";
    }
}

template <typename Writer, WriteProfile profile>
void write(Writer& writer, const Value& node, int depth = 0)
{
    switch (node.type()) {
        case Type::Null:
            writer << "null";
            break;
        case Type::False:
            writer << "false";
            break;
        case Type::True:
            writer << "true";
            break;
        case Type::I64:
            writer << std::format("{}", node.get_i64());
            break;
        case Type::F64:
            writer << std::format("{}", node.get_f64());
            break;
        case Type::String:
            writer << string_to_literal(node.get_string());
            break;
        case Type::Array: {
            writer << "[";
            auto first = true;
            for (const auto& child : node.get_underlying_array()) {
                if (!first) {
                    writer << ",";
                }
                first = false;

                if constexpr (profile == WriteProfile::Pretty) {
                    writer << "\n";
                    write_indent(writer, depth + 1);
                }
                write<Writer, profile>(writer, *child, depth + 1);
            }
            if constexpr (profile == WriteProfile::Pretty) {
                writer << "\n";
                write_indent(writer, depth);
            }
            writer << "]";
            break;
        }
        case Type::Object: {
            writer << "{";
            auto first = true;
            for (const auto& [key, child] : node.get_underlying_object()) {
                if (!first) {
                    writer << ",";
                }
                first = false;

                if constexpr (profile == WriteProfile::Pretty) {
                    writer << "\n";
                    write_indent(writer, depth + 1);
                }
                writer << string_to_literal(key);
                writer << ":";
                if constexpr (profile == WriteProfile::Pretty) {
                    writer << " ";
                }
                write<Writer, profile>(writer, *child, depth + 1);
            }
            if constexpr (profile == WriteProfile::Pretty) {
                writer << "\n";
                write_indent(writer, depth);
            }
            writer << "}";
            break;
        }
    }
}

}

namespace mst::json {

auto Value::clone() const -> std::unique_ptr<Value>
{
    switch (type()) {
        case Type::Null:
            return make_null();
        case Type::False:
            return make_bool(false);
        case Type::True:
            return make_bool(true);
        case Type::I64:
            return make_i64(get_i64());
        case Type::F64:
            return make_f64(get_f64());
        case Type::String:
            return make_string(get_string());
        case Type::Array: {
            auto array = Array();
            for (const auto& val : get_underlying_array()) {
                array.push_back(val->clone());
            }
            return std::make_unique<Value>(Type::Array, Data(std::move(array)));
        }
        case Type::Object: {
            auto object = Object();
            for (const auto& [key, val] : get_underlying_object()) {
                object[key] = val->clone();
            }
            return std::make_unique<Value>(
                Type::Object, Data(std::move(object)));
        }
    }
    std::unreachable();
}

auto Value::query(std::string_view path) & -> Result<Value*, std::string>
{
    auto parser = query::Parser(path);
    return query::resolve(parser, *this);
}

auto Value::query(
    std::string_view path) const& -> Result<const Value*, std::string>
{
    auto parser = query::Parser(path);
    return query::resolve(parser, *this);
}

auto Value::write(std::ostream& stream, WriteProfile profile) const
{
    if (profile == WriteProfile::Minified) {
        stringify::write<std::ostream, WriteProfile::Minified>(stream, *this);
    } else {
        stringify::write<std::ostream, WriteProfile::Pretty>(stream, *this);
    }
}

auto Value::to_string(WriteProfile profile) const -> std::string
{
    auto stream = std::stringstream();
    if (profile == WriteProfile::Minified) {
        stringify::write<std::stringstream, WriteProfile::Minified>(
            stream, *this);
    } else {
        stringify::write<std::stringstream, WriteProfile::Pretty>(
            stream, *this);
    }
    return stream.str();
}

auto parse(std::string_view text) -> Result<std::unique_ptr<Value>>
{
    return parse::Parser(text).parse();
}

}
