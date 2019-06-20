use crate::ast::Expr;
use crate::bytecode::Code;
use std::collections::HashMap;

pub trait Compile {
    fn compile(&self, addresses: &HashMap<String, u8>) -> Vec<Code>;
}

impl Compile for Expr {
    fn compile(&self, addresses: &HashMap<String, u8>) -> Vec<Code> {
        match self {
            Expr::Const(val) => vec![Code::Put(*val)],
            Expr::Var(symbol) => vec![Code::Load(addresses[symbol])],
            Expr::And(a, b) => {
                let mut result = a.compile(&addresses);
                result.extend(b.compile(&addresses).into_iter());
                result.push(Code::And);
                result
            }
            Expr::Or(a, b) => {
                let mut result = a.compile(&addresses);
                result.extend(b.compile(&addresses).into_iter());
                result.push(Code::Or);
                result
            }
            Expr::Impl(a, b) => {
                let mut result = a.compile(&addresses);
                result.extend(b.compile(&addresses).into_iter());
                result.push(Code::Impl);
                result
            }
            Expr::Biimpl(a, b) => {
                let mut result = a.compile(&addresses);
                result.extend(b.compile(&addresses).into_iter());
                result.push(Code::Biimpl);
                result
            }
            Expr::Not(a) => {
                let mut result = a.compile(&addresses);
                result.push(Code::Not);
                result
            }
        }
    }
}
