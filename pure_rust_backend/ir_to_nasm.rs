use crate::IR;
use std::collections::HashMap;

pub struct CodeGen {
    label_id: usize,
    var_offset: HashMap<String, i32>,
    next_offset: i32,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            label_id: 0,
            var_offset: HashMap::new(),
            next_offset: -8, // first local = [rbp-8]
        }
    }

    fn new_label(&mut self) -> String {
        let id = self.label_id;
        self.label_id += 1;
        format!(".L{}", id)
    }

    // core codegen entry
    pub fn gen(&mut self, ir: &IR) -> String {
        match ir {
            // ------------------------------------------------------------
            // Literals
            // ------------------------------------------------------------
            IR::Num(n) => format!("    mov rax, {}\n", n),

            IR::Str(s) => format!(
                "    ; string literal '{}'\n    mov rax, 0\n",
                s
            ),

            // ------------------------------------------------------------
            // Variables
            // ------------------------------------------------------------
            IR::Var(name) => {
                let offset = self.var_offset[name];
                format!("    mov rax, [rbp{}]\n", offset)
            }

            IR::Let { name, value } => {
                let prev_offset = self.next_offset;
                self.var_offset.insert(name.clone(), prev_offset);
                self.next_offset -= 8;

                let value_code = self.gen(value);
                format!("{value_code}    mov [rbp{}], rax\n", prev_offset)
            }

            // ------------------------------------------------------------
            // Binary Ops
            // ------------------------------------------------------------
            IR::Add(a, b) => binop(self, a, b, "add"),
            IR::Sub(a, b) => binop(self, a, b, "sub"),
            IR::Mul(a, b) => binop(self, a, b, "imul"),

            IR::Div(a, b) => {
                let left = self.gen(a);
                let right = self.gen(b).replace("rax", "rbx");
                format!("{left}{right}    mov rdx, 0\n    idiv rbx\n")
            }

            // ------------------------------------------------------------
            // Comparisons
            // ------------------------------------------------------------
            IR::Eq(a, b) => cmpop(self, a, b, "sete"),
            IR::Lt(a, b) => cmpop(self, a, b, "setl"),
            IR::Gt(a, b) => cmpop(self, a, b, "setg"),

            // ------------------------------------------------------------
            // Blocks with offset restore
            // ------------------------------------------------------------
            IR::Block(exprs) => {
                let saved = self.next_offset;
                let saved_vars = self.var_offset.clone();

                let mut out = String::new();
                for e in exprs {
                    out.push_str(&self.gen(e));
                }

                // block end → restore locals
                self.var_offset = saved_vars;
                self.next_offset = saved;
                out
            }

            // ------------------------------------------------------------
            // If Control Flow
            // ------------------------------------------------------------
            IR::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let lbl_else = self.new_label();
                let lbl_end = self.new_label();

                let c = self.gen(cond);
                let t = self.gen(then_branch);
                let e = self.gen(else_branch);

                format!(
                    "{c}    cmp rax, 0\n    je {lbl_else}\n{t}    jmp {lbl_end}\n{lbl_else}:\n{e}{lbl_end}:\n"
                )
            }

            // ------------------------------------------------------------
            // Function Definition
            // ------------------------------------------------------------
            IR::FuncDef { name, params, body } => {
                let mut local = CodeGen::new();
                let ret_label = local.new_label();

                // register → rbp-offset mapping for params
                let mut offsets = 16;
                for p in params {
                    local
                        .var_offset
                        .insert(p.clone(), offsets);
                    offsets += 8;
                }

                let body_code = local.gen(body);

                format!(
                    "global {name}\n{name}:\n    push rbp\n    mov rbp, rsp\n{body_code}\n{ret_label}:\n    pop rbp\n    ret\n"
                )
            }

            // ------------------------------------------------------------
            // Function Call
            // ------------------------------------------------------------
            IR::Call { name, args } => {
                let regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                let mut out = String::new();

                for (i, a) in args.iter().enumerate() {
                    out.push_str(&self.gen(a));
                    out.push_str(&format!("    mov {}, rax\n", regs[i]));
                }

                out.push_str(&format!("    call {name}\n"));
                out.push_str("    ; rax = return\n\n");
                out
            }

            // ------------------------------------------------------------
            // Return
            // ------------------------------------------------------------
            IR::Return(expr) => {
                let code = self.gen(expr);
                format!("{code}    jmp .L_return\n")
            }
        }
    }
}

// ============================================================================
// Binary Operation Helper
// ============================================================================
fn binop(g: &mut CodeGen, a: &IR, b: &IR, op: &str) -> String {
    let left = g.gen(a);
    let right = g.gen(b).replace("rax", "rbx");
    format!("{left}{right}    {op} rax, rbx\n")
}


// ============================================================================
// Comparison Helper
// ============================================================================
fn cmpop(g: &mut CodeGen, a: &IR, b: &IR, set: &str) -> String {
    let left = g.gen(a);
    let right = g.gen(b).replace("rax", "rbx");
    format!("{left}{right}    cmp rax, rbx\n    {set} al\n    movzx rax, al\n")
}
