use mojo_like_rust::IR;

fn ir_to_cpp(ir: &IR) -> String {
    match ir {
        IR::Num(n) => format!("Number<{}>", n),
        IR::Add(a, b) => format!("Add<{},{}>", ir_to_cpp(a), ir_to_cpp(b)),
        _ => unimplemented!(),
    }
}

fn main() {
    let json = std::fs::read_to_string("ir_output.json").unwrap();
    let ir: IR = serde_json::from_str(&json).unwrap();

    let mut out = String::new();
    out.push_str("#pragma once\n#include \"../meta_nasm/meta_ast.hpp\"\n\n");
    out.push_str("using GeneratedExpr = ");
    out.push_str(&ir_to_cpp(&ir));
    out.push_str(";\n");

    std::fs::write("bridge/generated_meta_ast.hpp", out).unwrap();
}
