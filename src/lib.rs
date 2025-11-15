use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IR {
    Num(i64),
    Str(String),

    Var(String),
    Let { name: String, value: Box<IR> },

    Add(Box<IR>, Box<IR>),
    Sub(Box<IR>, Box<IR>),
    Mul(Box<IR>, Box<IR>),
    Div(Box<IR>, Box<IR>),

    Eq(Box<IR>, Box<IR>),
    Lt(Box<IR>, Box<IR>),
    Gt(Box<IR>, Box<IR>),

    Block(Vec<IR>),

    If {
        cond: Box<IR>,
        then_branch: Box<IR>,
        else_branch: Box<IR>,
    },

    FuncDef {
        name: String,
        params: Vec<String>,
        body: Box<IR>,
    },

    Call {
        name: String,
        args: Vec<IR>,
    },

    Return(Box<IR>),
}
