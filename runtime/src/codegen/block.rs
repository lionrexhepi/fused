use parser::ast::{ block::Block, statements::{ Statement, StatementContent } };

use crate::codegen::{ ToBytecode, Codegen };

use super::CodegenResult;

impl ToBytecode for Statement {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult {
        match &self.content {
            StatementContent::Expr(expr) => expr.to_bytecode(codegen),
            StatementContent::Module(_) => todo!(),
            StatementContent::Use(_) => todo!(),
        }
    }
}

impl ToBytecode for Block {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult {
        codegen.new_scope(|codegen: &mut Codegen| {
            for statement in &self.0 {
                statement.to_bytecode(codegen)?;
                println!("Codegen len after statememt {statement:?}: {}", codegen.bytes.len());
            }

            Ok(())
        })
    }
}
