mod lexer;
mod parser;
mod ast;
mod vm;

use lexer::lex;
use parser::Parser;
use vm::VM;

fn main() {
    let src = r#"
fn add(a, b)
    return a + b

fn main()
    x = add(3, 7)
    return x
"#;

    let tokens = lex(src);
    let mut parser = Parser::new(tokens);

    let mut vm = VM::new();

    let f1 = parser.parse_function();
    let f2 = parser.parse_function();

    vm.functions.insert(f1.name.clone(), f1);
    vm.functions.insert(f2.name.clone(), f2);

    let result = vm.run_function("main", vec![]);
    println!("Result = {}", result);
}
