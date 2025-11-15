use mojo_like_rust::IR;
use serde_json;

fn main() {
    // 샘플 IR: (3 + 7)
    let ir = IR::Add(Box::new(IR::Num(3)), Box::new(IR::Num(7)));

    let json = serde_json::to_string_pretty(&ir).unwrap();
    std::fs::write("ir_output.json", json).unwrap();

    println!("IR saved → ir_output.json");
}
