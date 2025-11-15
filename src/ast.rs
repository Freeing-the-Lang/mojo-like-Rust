// mojo-like-rust / src/ast.rs

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Ident(String),
    Call { name: String, args: Vec<Expr> },
    Block(Vec<Expr>),
    Return(Box<Expr>),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Expr,
}
