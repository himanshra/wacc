use crate::parser::{Expr, Program, Stmt};

pub struct Codegen {}

impl Codegen {
    pub fn generate(program: &Program) -> String {
        let mut out = String::new();

        let fn_name = program.function.name.clone();

        out.push_str(format!(".globl _{}\n", fn_name).as_str());
        out.push_str(format!("_{}:\n", fn_name).as_str());

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
