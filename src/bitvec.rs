#[derive(Copy, Clone, Debug)]
pub struct Bitvec(u128);

impl Bitvec {
    pub fn new(values: u128) -> Bitvec {
        Bitvec(values)
    }

    pub fn get(&self, idx: u8) -> bool {
        self.0 & (1 << idx) > 0
    }
}
