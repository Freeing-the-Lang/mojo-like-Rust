#include <fstream>
#include "meta_codegen.hpp"
#include "../bridge/generated_meta_ast.hpp"

int main() {
    auto code = generate_nasm<GeneratedExpr>();

    std::ofstream out("nasm_output.asm");
    for (auto c : code) out << c;

    return 0;
}
