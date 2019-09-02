use crate::Bitvec;

#[derive(Debug)]
pub struct Output {
    pub symbols: Vec<String>,
    pub equation: String,
    pub results: Vec<bool>,
}

impl Output {
    pub fn fmt_csv(&self) -> String {
        fn string(b: bool) -> &'static str {
            if b {
                "1"
            } else {
                "0"
            }
        }

        let mut s = self.symbols.join(",") + "," + &self.equation + "\n";
        for (result, values) in self.results.iter().zip((0..).map(Bitvec::new)) {
            for idx in 0..self.symbols.len() as u8 {
                s += string(values.get(idx));
                s += ",";
            }

            s += &(result.to_string() + "\n");
        }

        s
    }
}
