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
        codegen.new_scope(|codegen: &mut Codegen| {
            let mut result = 0;
            for statement in &self.0 {
                result = statement.to_bytecode(codegen);
            }
            result
        })
    }
}
