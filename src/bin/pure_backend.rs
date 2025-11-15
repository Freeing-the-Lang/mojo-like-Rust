use mojo_like_rust::IR;

fn gen(ir: &IR) -> String {
    match ir {
        IR::Num(n) => format!("    mov rax, {}\n", n),
        IR::Add(a, b) => format!("{}{}    add rax, rbx\n",
            gen(a),
            gen(b).replace("rax","rbx")
        ),
        _ => unimplemented!()
    }
}

fn main() {
    let json = std::fs::read_to_string("ir_output.json").unwrap();
    let ir: IR = serde_json::from_str(&json).unwrap();
    let asm = gen(&ir);
    std::fs::write("pure_rust_output.asm", asm).unwrap();
}
