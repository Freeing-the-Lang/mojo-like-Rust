// mojo-like-rust / src/vm.rs

use crate::ast::{Expr, Function};
use std::collections::HashMap;

pub struct VM {
    pub functions: HashMap<String, Function>,
    pub vars: HashMap<String, i64>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            vars: HashMap::new(),
        }
    }

    pub fn run_function(&mut self, name: &str, args: Vec<i64>) -> i64 {
        let f = self.functions.get(name).expect("function not found");

        for (i, p) in f.params.iter().enumerate() {
            self.vars.insert(p.clone(), args[i]);
        }

        self.eval(&f.body)
    }

    fn eval(&mut self, expr: &Expr) -> i64 {
        match expr {
            Expr::Number(n) => *n,
            Expr::Ident(s) => *self.vars.get(s).unwrap(),

            Expr::Return(e) => self.eval(e),

            Expr::Block(items) => {
                let mut last = 0;
                for e in items {
                    last = self.eval(e);
                }
                last
            }

            Expr::Call { name, args } => {
                let evaled: Vec<i64> = args.iter().map(|e| self.eval(e)).collect();
                self.run_function(name, evaled)
            }
        }
    }
}
