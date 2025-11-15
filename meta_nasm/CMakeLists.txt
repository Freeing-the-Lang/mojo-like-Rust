cmake_minimum_required(VERSION 3.16)
project(meta_nasm_codegen LANGUAGES CXX)

# ----------------------------
# C++20 / 23 모드
# ----------------------------
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# ----------------------------
# Debug / Release 설정
# ----------------------------
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()

# ----------------------------
# 컴파일러별 최적화
# ----------------------------
if (MSVC)
    # Windows MSVC
    add_compile_options(/O2 /Ob2 /DNDEBUG)
else()
    # GCC / Clang
    add_compile_options(-O2 -march=native -DNDEBUG)
endif()

# ----------------------------
# meta_nasm 실행 프로그램 정의
# ----------------------------
add_executable(meta_nasm
    main.cpp
    meta_codegen.hpp
    meta_ast.hpp
    meta_nasm_builder.hpp
)

# ----------------------------
# 실행 후 nasm_output.asm 생성 메시지
# ----------------------------
add_custom_command(
    TARGET meta_nasm POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E echo "✔ meta_nasm 빌드 완료!"
    COMMAND ${CMAKE_COMMAND} -E echo "✔ meta_nasm 실행 → nasm_output.asm 생성"
)
