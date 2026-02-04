use crate::parser::{Expr, Program, Stmt};

pub struct Codegen {}

impl Codegen {
    pub fn generate(program: &Program) -> String {
        let mut out = String::new();

        out.push_str(".globl _main\n");
        out.push_str("_main:\n");

        match &program.function.body {
            Stmt::Return(expr) => match expr {
                Expr::Constant(value) => {
                    out.push_str(&format!("    mov w0, #{}\n", value));
                    out.push_str("    ret\n");
                }
            },
        }

        out
    }
}
