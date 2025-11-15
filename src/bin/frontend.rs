use mojo_like_rust::IR;
use serde_json;

fn main() {
    let ir = IR::Add(Box::new(IR::Num(3)), Box::new(IR::Num(7)));
    let json = serde_json::to_string(&ir).unwrap();
    std::fs::write("ir_output.json", json).unwrap();
}
