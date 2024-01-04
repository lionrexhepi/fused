use parser::ast::{ expr::{ Expr, ExprLit }, simple::{ ExprSimple, BinaryType }, number::Number };

use crate::{ stack::{ Register, RegisterContents }, instructions::Instruction };

use super::{ ToBytecode, Codegen };

impl ToBytecode for ExprLit {
    fn to_bytecode(&self, codegen: &mut Codegen) -> Register {
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

impl ToBytecode for ExprSimple {
    fn to_bytecode(&self, codegen: &mut Codegen) -> Register {
        match self {
            ExprSimple::Binary(left, op, right) => {
                let left = left.to_bytecode(codegen);
                let right = right.to_bytecode(codegen);
                match op {
                    BinaryType::Assign => todo!(),
                    BinaryType::AddAssign => todo!(),
                    BinaryType::SubAssign => todo!(),
                    BinaryType::MulAssign => todo!(),
                    BinaryType::DivAssign => todo!(),
                    BinaryType::ModAssign => todo!(),
                    BinaryType::AndAssign => todo!(),
                    BinaryType::OrAssign => todo!(),
                    BinaryType::BitAndAssign => todo!(),
                    BinaryType::BitOrAssign => todo!(),
                    BinaryType::BitXorAssign => todo!(),
                    BinaryType::LeftShiftAssign => todo!(),
                    BinaryType::RightShiftAssign => todo!(),
                    BinaryType::Or => codegen.emit_or(left, right),
                    BinaryType::Add => codegen.emit_add(left, right),
                    BinaryType::Sub => codegen.emit_sub(left, right),
                    BinaryType::Mul => codegen.emit_mul(left, right),
                    BinaryType::Div => codegen.emit_div(left, right),
                    BinaryType::Mod => codegen.emit_mod(left, right),
                    BinaryType::BitAnd => codegen.emit_bitand(left, right),
                    BinaryType::And => codegen.emit_and(left, right),
                    BinaryType::BitOr => codegen.emit_bitor(left, right),
                    BinaryType::BitXor => codegen.emit_bitxor(left, right),
                    BinaryType::Eq => codegen.emit_eq(left, right),
                    BinaryType::Neq => todo!(),
                    BinaryType::Lt => todo!(),
                    BinaryType::Gt => todo!(),
                    BinaryType::Leq => todo!(),
                    BinaryType::Geq => todo!(),
                    BinaryType::LeftShift => codegen.emit_leftshift(left, right),
                    BinaryType::RightShift => codegen.emit_rightshift(left, right),
                }
            }
            ExprSimple::Literal(lit) => lit.to_bytecode(codegen),
            _ => todo!("ee"),
        }
    }
}

impl ToBytecode for Expr {
    fn to_bytecode(&self, codegen: &mut Codegen) -> Register {
        match self {
            Expr::Simple(simple) => simple.to_bytecode(codegen),
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
    use libimmixcons::{ immix_init, immix_noop_callback };
    use parser::{ tokens::stream::TokenStream, ast::{ stream::ParseStream, expr::Expr } };

    use crate::{ codegen::ToBytecode, Thread, stack::Stack };

    use super::Codegen;

    #[test]
    fn test_add() {
        let tokens = TokenStream::from_string("5 + 3").unwrap();
        let mut stream = ParseStream::new(tokens);
        let expr = stream.parse::<Expr>().unwrap();
        println!("{:?}", expr);
        let mut codegen = Codegen::new();
        let result = expr.to_bytecode(&mut codegen);
        codegen.emit_return(result);
        let chunk = codegen.chunk();

        immix_init(512 * 1000, 0, immix_noop_callback, core::ptr::null_mut());
        let mut thread = Thread {
            stack: Stack::new(),
        };

        let result = thread.run_chunk(chunk).unwrap();
        println!("result: {result}")
    }
}
