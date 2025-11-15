#include "meta_codegen.hpp"
#include <fstream>

using Expr = Add<Number<3>, Number<7>>;

// compile-time nasm assembly
constexpr auto code = generate_nasm<Expr>();

int main() {
    std::ofstream out("nasm_output.asm");

    for(auto c : code) {
        out << c;
    }

    return 0;
}
