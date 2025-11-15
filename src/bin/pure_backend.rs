use mojo_like_rust::IR;
mod ir_to_nasm;
use ir_to_nasm::CodeGen;

fn main() {
    let json = std::fs::read_to_string("ir_output.json").unwrap();
    let ir: IR = serde_json::from_str(&json).unwrap();

    let mut gen = CodeGen::new();
    let asm = gen.gen(&ir);

    std::fs::write("pure_rust_output.asm", asm).unwrap();

    println!("Pure Rust backend output → pure_rust_output.asm");
}
