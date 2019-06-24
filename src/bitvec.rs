#[derive(Copy, Clone, Debug)]
pub struct Bitvec(usize);

impl Bitvec {
    pub fn new(values: usize) -> Bitvec {
        Bitvec(values)
    }

    pub fn get(&self, idx: u8) -> bool {
        self.0 & (1 << idx) > 0
    }
}
