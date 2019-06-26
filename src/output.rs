use crate::Bitvec;

#[derive(Debug)]
pub struct Output {
    pub symbols: Vec<String>,
    pub equation: String,
    pub results: Vec<bool>,
}

impl Output {
    pub fn fmt_csv(&self) -> String {
        let mut s = self.symbols.join(",") + "," + &self.equation + "\n";
        for (result, values) in self.results.iter().zip((0..).map(Bitvec::new)) {
            for idx in 0..self.symbols.len() as u8 {
                s += &(values.get(idx).to_string() + ",");
            }

            s += &(result.to_string() + "\n");
        }

        s
    }
}
