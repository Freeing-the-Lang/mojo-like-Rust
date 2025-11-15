pub enum IR {
    Num(i64),
    Add(Box<IR>, Box<IR>),
    Mul(Box<IR>, Box<IR>),
    Sub(Box<IR>, Box<IR>),
    Div(Box<IR>, Box<IR>),
    Block(Vec<IR>),
}
