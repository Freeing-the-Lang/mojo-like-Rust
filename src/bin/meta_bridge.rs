use mojo_like_rust::IR;

fn ir_to_cpp(ir: &IR) -> String {
    match ir {
        IR::Num(n) => format!("Number<{}>", n),
        IR::Add(a, b) => format!("Add<{}, {}>", ir_to_cpp(a), ir_to_cpp(b)),
        IR::Sub(a, b) => format!("Sub<{}, {}>", ir_to_cpp(a), ir_to_cpp(b)),
        IR::Mul(a, b) => format!("Mul<{}, {}>", ir_to_cpp(a), ir_to_cpp(b)),
        IR::Div(a, b) => format!("Div<{}, {}>", ir_to_cpp(a), ir_to_cpp(b)),
        IR::Return(e) => format!("Return<{}>", ir_to_cpp(e)),
        IR::Block(list) => {
            let inner = list
                .iter()
                .map(ir_to_cpp)
                .collect::<Vec<_>>()
                .join(", ");
            format!("Block<{}>", inner)
        }
        _ => unimplemented!("IR variant not implemented in meta bridge"),
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
