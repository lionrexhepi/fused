use parser::ast::{
    expr::{ ExprLit, Expr },
    number::{ LitNumber, Number },
    simple::{ ExprSimple, BinaryType },
    block::Block,
    statements::Statement,
};

use crate::{ codegen::{ ToBytecode, Codegen }, stack::{ Register, RegisterContents } };

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
            ExprSimple::Binary(left, op, right) if *op == BinaryType::Add => {
                let left = left.to_bytecode(codegen);
                let right = right.to_bytecode(codegen);
                codegen.emit_add(left, right)
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

impl ToBytecode for Statement {
    fn to_bytecode(&self, codegen: &mut Codegen) -> Register {
        match &self.content {
            parser::ast::statements::StatementContent::Expr(expr) => expr.to_bytecode(codegen),
            parser::ast::statements::StatementContent::Module(_) => todo!(),
            parser::ast::statements::StatementContent::Use(_) => todo!(),
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
