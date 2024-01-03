use parser::ast::{ block::Block, statements::{ Statement, StatementContent } };

use crate::{ codegen::{ ToBytecode, Codegen }, stack::Register };

impl ToBytecode for Statement {
    fn to_bytecode(&self, codegen: &mut Codegen) -> Register {
        match &self.content {
            StatementContent::Expr(expr) => expr.to_bytecode(codegen),
            StatementContent::Module(_) => todo!(),
            StatementContent::Use(_) => todo!(),
        }
    }
}

impl ToBytecode for Block {
    fn to_bytecode(&self, codegen: &mut Codegen) -> Register {
        let len = self.0.len();
        let mut result = 0;
        for (i, expr) in self.0.iter().enumerate() {
            let expr_result = expr.to_bytecode(codegen);
            if i == len - 1 {
                result = expr_result;
            }
        }

        result
    }
}
