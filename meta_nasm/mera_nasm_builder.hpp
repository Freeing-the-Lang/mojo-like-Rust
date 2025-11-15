#pragma once
#include <string_view>
#include <array>

// Compile-time string builder
template<size_t N>
struct MetaString {
    std::array<char, N> buf {};

    constexpr MetaString(const char(&s)[N]) {
        for(size_t i = 0; i < N; i++)
            buf[i] = s[i];
    }

    constexpr auto str() const {
        return buf;
    }
};

// meta concatenation
template<size_t N1, size_t N2>
constexpr auto meta_cat(const MetaString<N1>& a, const MetaString<N2>& b) {
    std::array<char, N1 + N2 - 1> out{};
    for(size_t i = 0; i < N1 - 1; i++) out[i] = a.buf[i];
    for(size_t i = 0; i < N2; i++) out[N1 - 1 + i] = b.buf[i];
    return out;
}
