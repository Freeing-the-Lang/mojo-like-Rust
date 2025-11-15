use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IR {
    // Literals
    Num(i64),
    Str(String),

    // Variables
    Var(String),
    Let { name: String, value: Box<IR> },

    // Binary operations
    Add(Box<IR>, Box<IR>),
    Sub(Box<IR>, Box<IR>),
    Mul(Box<IR>, Box<IR>),
    Div(Box<IR>, Box<IR>),

    // Logic
    Eq(Box<IR>, Box<IR>),
    Lt(Box<IR>, Box<IR>),
    Gt(Box<IR>, Box<IR>),

    // Blocks / Scope
    Block(Vec<IR>),

    // Control flow
    If {
        cond: Box<IR>,
        then_branch: Box<IR>,
        else_branch: Box<IR>,
    },

    // Function call
    Call {
        name: String,
        args: Vec<IR>,
    },

    // Function definition
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Box<IR>,
    },

    // Return
    Return(Box<IR>),
}
