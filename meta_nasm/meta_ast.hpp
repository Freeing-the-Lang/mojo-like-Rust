#pragma once
#include <string>
#include <type_traits>

// --- AST Types ---------------------------------------------------

template<long N>
struct Number {
    static constexpr long value = N;
};

template<typename L, typename R>
struct Add {};

template<typename L, typename R>
struct Sub {};

template<typename L, typename R>
struct Mul {};

template<typename L, typename R>
struct Div {};

template<typename... Body>
struct Block {};

template<typename Name, typename... Args>
struct Call {};

template<typename Name>
struct Func {};

template<typename Name, typename Body>
struct FunctionDef {};
