// mojo-like-rust / src/parser.rs

use crate::lexer::{Token, TokenKind};
use crate::ast::{Expr, Function};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &TokenKind {
        &self.tokens[self.pos].kind
    }

    fn next(&mut self) -> &TokenKind {
        self.pos += 1;
        &self.tokens[self.pos-1].kind
    }

    pub fn parse_function(&mut self) -> Function {
        match self.next() {
            TokenKind::Fn => {},
            _ => panic!("expected fn"),
        }

        let name = match self.next() {
            TokenKind::Ident(s) => s.clone(),
            _ => panic!("expected function name"),
        };

        // (...)
        let mut params = Vec::new();
        self.expect(TokenKind::LParen);
        while !matches!(self.peek(), TokenKind::RParen) {
            if let TokenKind::Ident(s) = self.next() {
                params.push(s.clone());
            }
            if matches!(self.peek(), TokenKind::Comma) {
                self.next();
            }
        }
        self.expect(TokenKind::RParen);

        let body = self.parse_block();
        Function { name, params, body }
    }

    fn parse_block(&mut self) -> Expr {
        // newline -> indent -> block items -> dedent
        self.consume_newlines();
        self.expect(TokenKind::Indent);

        let mut exprs = Vec::new();

        while !matches!(self.peek(), TokenKind::Dedent | TokenKind::Eof) {
            exprs.push(self.parse_expr());
            self.consume_newlines();
        }

        self.expect(TokenKind::Dedent);
        Expr::Block(exprs)
    }

    fn parse_expr(&mut self) -> Expr {
        match self.next() {
            TokenKind::Number(n) => Expr::Number(n.parse().unwrap()),
            TokenKind::Ident(name) => {
                if matches!(self.peek(), TokenKind::LParen) {
                    self.next(); // (
                    let mut args = Vec::new();
                    while !matches!(self.peek(), TokenKind::RParen) {
                        args.push(self.parse_expr());
                        if matches!(self.peek(), TokenKind::Comma) {
                            self.next();
                        }
                    }
                    self.expect(TokenKind::RParen);
                    Expr::Call { name: name.clone(), args }
                } else {
                    Expr::Ident(name.clone())
                }
            }
            TokenKind::Return => {
                let e = self.parse_expr();
                Expr::Return(Box::new(e))
            }
            _ => panic!("unexpected token in expr"),
        }
    }

    fn expect(&mut self, k: TokenKind) {
        let t = self.next().clone();
        if t != k {
            panic!("expected {:?}, got {:?}", k, t);
        }
    }

    fn consume_newlines(&mut self) {
        while matches!(self.peek(), TokenKind::Newline) {
            self.next();
        }
    }
}
