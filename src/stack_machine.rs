use crate::bytecode::Code;

pub struct StackMachine<'a> {
    values: u128,
    operations: &'a Vec<Code>,
    stack: Vec<bool>,
}

impl<'a> StackMachine<'a> {
    pub fn new(values: u128, operations: &'a Vec<Code>) -> StackMachine<'a> {
        StackMachine {
            values,
            operations,
            stack: Vec::new(),
        }
    }

    fn push(&mut self, value: bool) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> bool {
        self.stack.pop().expect("Failed to pop from empty stack")
    }

    pub fn evaluate(mut self) -> bool {
        use Code::*;
        for op in self.operations.into_iter() {
            match *op {
                And => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a && b);
                }
                Or => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a || b);
                }
                Impl => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(!a && b);
                }
                Biimpl => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a == b);
                }
                Not => {
                    let a = self.pop();
                    self.push(!a);
                }
                Put(v) => self.push(v),
                Load(addr) => self.push(self.values & (1 << addr) > 0),
            }
        }

        self.pop()
    }
}
