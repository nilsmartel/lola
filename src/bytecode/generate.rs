use crate::ast::Expr;
use crate::bytecode::Code;
use std::collections::HashMap;

pub trait GenCode {
    fn gen_code(&self, addresses: &HashMap<String, u8>) -> Vec<Code>;
}

impl GenCode for Expr {
    fn gen_code(&self, addresses: &HashMap<String, u8>) -> Vec<Code> {
        match self {
            Expr::Const(val) => vec![Code::Put(*val)],
            Expr::Var(symbol) => vec![Code::Load(addresses[symbol])],
            Expr::And(a, b) => {
                let mut result = a.gen_code(&addresses);
                result.extend(b.gen_code(&addresses).into_iter());
                result.push(Code::And);
                result
            }
            Expr::Or(a, b) => {
                let mut result = a.gen_code(&addresses);
                result.extend(b.gen_code(&addresses).into_iter());
                result.push(Code::Or);
                result
            }
            Expr::Impl(a, b) => {
                let mut result = a.gen_code(&addresses);
                result.extend(b.gen_code(&addresses).into_iter());
                result.push(Code::Impl);
                result
            }
            Expr::Biimpl(a, b) => {
                let mut result = a.gen_code(&addresses);
                result.extend(b.gen_code(&addresses).into_iter());
                result.push(Code::Biimpl);
                result
            }
            Expr::Not(a) => {
                let mut result = a.gen_code(&addresses);
                result.push(Code::Not);
                result
            }
        }
    }
}
