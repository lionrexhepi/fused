use parser::ast::{
    declarations::ExprDecl,
    expr::{Expr, ExprLit},
    number::Number,
    simple::{BinaryType, ExprSimple}, path::PathSegment,
};

use crate::{instructions::Instruction, stack::RegisterContents};

use super::{Codegen, CodegenResult, ToBytecode};

impl ToBytecode for ExprLit {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult {
        let value = match self {
            ExprLit::String(_) => todo!(),
            ExprLit::Number(num) => match num.number {
                Number::Int(int) => RegisterContents::Int(int),
                Number::Float(float) => RegisterContents::Float(float),
                Number::UInt(uint) => RegisterContents::Int(uint as i64),
            },
            ExprLit::Bool(bool) => RegisterContents::Bool(bool.value),
        };

        Ok(codegen.emit_const(value))
    }
}

impl ToBytecode for ExprDecl {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult {
        codegen.declare(self.name.name.clone(), self.mutable);
        if let Some(value) = &self.value {
            let value_reg = value.to_bytecode(codegen)?;
            codegen.emit_store(&self.name.name, value_reg)
        } else {
            Ok(0)
        }
    }
}

impl ToBytecode for ExprSimple {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult {
        match self {
            ExprSimple::Binary(left, op, right) => {
                let left = left.to_bytecode(codegen)?;
                let right = right.to_bytecode(codegen)?;
                let instruction = match op {
                    BinaryType::Or => Instruction::Or,
                    BinaryType::Add => Instruction::Add,
                    BinaryType::Sub => Instruction::Sub,
                    BinaryType::Mul => Instruction::Mul,
                    BinaryType::Div => Instruction::Div,
                    BinaryType::Mod => Instruction::Mod,
                    BinaryType::BitAnd => Instruction::BitAnd,
                    BinaryType::And => Instruction::And,
                    BinaryType::BitOr => Instruction::BitOr,
                    BinaryType::BitXor => Instruction::BitXor,
                    BinaryType::Eq => Instruction::Eq,
                    BinaryType::Neq => todo!(),
                    BinaryType::Lt => todo!(),
                    BinaryType::Gt => todo!(),
                    BinaryType::Leq => todo!(),
                    BinaryType::Geq => todo!(),
                    BinaryType::LeftShift => Instruction::LeftShift,
                    BinaryType::RightShift => Instruction::RightShift,
                };
                Ok(codegen.emit_binary(left, right, instruction))
            }
            ExprSimple::Literal(lit) => lit.to_bytecode(codegen),
            ExprSimple::Path(path) => {
                if path.segments.len() == 1 {
                    let ident = path.segments.last().unwrap();
                    if let PathSegment::Ident(ident) = ident {
                        codegen.emit_load(&ident.name)
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }
            _ => todo!("ee"),
        }
    }
}

impl ToBytecode for Expr {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult {
        match self {
            Expr::Simple(simple) => simple.to_bytecode(codegen),
            Expr::Decl(decl) => decl.to_bytecode(codegen),
            Expr::Function(_) => todo!("eeee"),
            Expr::If(_) => todo!("ees"),
            Expr::While(_) => todo!("sse"),
            Expr::For(_) => todo!("fe"),
            Expr::Loop(_) => todo!("dede"),
            Expr::Empty => todo!("öeö"),
        }
    }
}

#[cfg(test)]
mod test {
    use parser::{
        ast::{expr::Expr, stream::ParseStream},
        tokens::stream::TokenStream,
    };

    use crate::{codegen::ToBytecode, stack::Stack, Thread};

    use super::Codegen;

    #[test]
    fn test_add() {
        let tokens = TokenStream::from_string("5 + 3").unwrap();
        let mut stream = ParseStream::new(tokens);
        let expr = stream.parse::<Expr>().unwrap();
        println!("{:?}", expr);
        let mut codegen = Codegen::new();
        let result = expr.to_bytecode(&mut codegen).unwrap();
        codegen.emit_return(result);
        let chunk = codegen.chunk();

        let mut thread = Thread {
            stack: Stack::new(),
        };

        let result = thread.run_chunk(chunk).unwrap();
        println!("result: {result}")
    }
}
