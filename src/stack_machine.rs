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

    fn evaluate(mut self) -> bool {
        use Code::*;
        for op in self.operations.into_iter() {
            match *op {
                And => self.push(self.pop() && self.pop()),
                Or => self.push(self.pop() || self.pop()),
                Impl => self.push(!self.pop() && self.pop()),
                Biimpl => self.push(self.pop() == self.pop()),
                Not => self.push(!self.pop()),
                Put(v) => self.push(v),
                Load(addr) => self.push(self.values & (1 << addr) > 0),
            }
        }

        self.pop()
    }
}
