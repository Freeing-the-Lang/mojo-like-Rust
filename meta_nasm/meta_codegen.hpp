#pragma once
#include "meta_ast.hpp"
#include "meta_nasm_builder.hpp"
#include <type_traits>

// Convert integer to NASM immediate
template<long N>
constexpr auto to_nasm(Number<N>) {
    constexpr char buf[] = "    mov rax, ";
    return MetaString<sizeof(buf)>{buf};
}

// Addition
template<typename L, typename R>
constexpr auto to_nasm(Add<L,R>) {
    // rax = L + R
    constexpr auto s1 = to_nasm(L{});
    constexpr char add_ins[] = "    add rax, ";
    constexpr auto s2 = MetaString<sizeof(add_ins)>{add_ins};

    return meta_cat(s1, s2);
}

// Mul example
template<typename L, typename R>
constexpr auto to_nasm(Mul<L,R>) {
    // rax = L * R
    constexpr auto s1 = to_nasm(L{});
    constexpr char mul_ins[] = "    imul rax, ";
    constexpr auto s2 = MetaString<sizeof(mul_ins)>{mul_ins};
    return meta_cat(s1, s2);
}
