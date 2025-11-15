cmake_minimum_required(VERSION 3.16)
project(meta_nasm CXX)

set(CMAKE_CXX_STANDARD 20)

add_executable(meta_nasm
    main.cpp
    meta_ast.hpp
    meta_codegen.hpp
    meta_nasm_builder.hpp
)
