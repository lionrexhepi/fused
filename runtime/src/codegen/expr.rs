use parser::ast::{
    conditionals::{ Else, ExprIf },
    declarations::ExprDecl,
    expr::{ Expr, ExprLit },
    number::Number,
    path::PathSegment,
    simple::{ BinaryType, ExprSimple },
};

use crate::{ instructions::Instruction, stack::RegisterContents };

use super::{ Codegen, CodegenResult, ToBytecode };

impl ToBytecode for ExprLit {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult {
        let value = match self {
            ExprLit::String(_) => todo!(),
            ExprLit::Number(num) =>
                match num.number {
                    Number::Int(int) => RegisterContents::Int(int),
                    Number::Float(float) => RegisterContents::Float(float),
                    Number::UInt(uint) => RegisterContents::Int(uint as i64),
                }
            ExprLit::Bool(bool) => RegisterContents::Bool(bool.value),
        };

        codegen.emit_const(value)
    }
}

impl ToBytecode for ExprDecl {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult {
        codegen.declare(self.name.name.clone(), self.mutable);
        if let Some(value) = &self.value {
            value.to_bytecode(codegen)?;
            codegen.emit_store(&self.name.name)
        } else {
            Ok(())
        }
    }
}

impl ToBytecode for ExprSimple {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult {
        match self {
            ExprSimple::Binary(left, op, right) => {
                left.to_bytecode(codegen)?;
                right.to_bytecode(codegen)?;
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
                codegen.emit_simple(instruction)
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
            ExprSimple::Assign(target, value) => {
                if target.segments.len() == 1 {
                    let ident = target.segments.last().unwrap();
                    if let PathSegment::Ident(ident) = ident {
                        value.to_bytecode(codegen)?;
                        codegen.emit_store(&ident.name)
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }
            other => todo!("{other:?}"),
        }
    }
}

impl ToBytecode for ExprIf {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult {
        codegen.new_scope(|codegen| {
            self.condition.to_bytecode(codegen)?;
            let jump_to_else = codegen.emit_cond_jump();
            self.body.to_bytecode(codegen)?;
            if let Some(r#else) = &self.r#else {
                let skip_else = codegen.emit_uncond_jump();
                codegen.patch_jump(jump_to_else);
                r#else.to_bytecode(codegen)?;
                codegen.patch_jump(skip_else);
            } else {
                codegen.patch_jump(jump_to_else);
            }

            Ok(())
        })
    }
}

impl ToBytecode for Else {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult {
        match self {
            Else::If(r#if) => r#if.to_bytecode(codegen),
            Else::Body(block) => { block.to_bytecode(codegen) }
        }
    }
}

impl ToBytecode for Expr {
    fn to_bytecode(&self, codegen: &mut Codegen) -> CodegenResult {
        match self {
            Expr::Simple(simple) => simple.to_bytecode(codegen),
            Expr::Decl(decl) => decl.to_bytecode(codegen),
            Expr::Function(_) => todo!("eeee"),
            Expr::If(r#if) => r#if.to_bytecode(codegen),
            Expr::While(_) => todo!("sse"),
            Expr::For(_) => todo!("fe"),
            Expr::Loop(_) => todo!("dede"),
            Expr::Empty => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use parser::{ ast::{ expr::Expr, stream::ParseStream }, tokens::stream::TokenStream };

    use crate::{ codegen::ToBytecode, instructions::Instruction, stack::Stack, Thread };

    use super::Codegen;

    #[test]
    fn test_add() {
        let tokens = TokenStream::from_string("5 + 3").unwrap();
        let mut stream = ParseStream::new(tokens);
        let expr = stream.parse::<Expr>().unwrap();
        let mut codegen = Codegen::new();
        expr.to_bytecode(&mut codegen).unwrap();
        codegen.emit_simple(Instruction::Return).unwrap();
        let chunk = codegen.chunk();

        let mut thread = Thread {
            stack: Stack::new(),
        };

        let result = thread.run_chunk(chunk).unwrap();
        println!("result: {result}")
    }
}
