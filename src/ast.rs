use std::collections::HashSet;

type BE = Box<Expr>;

pub enum Expr {
    And(BE, BE),
    Or(BE, BE),
    Impl(BE, BE),
    Biimpl(BE, BE),
    Not(BE),
    Var(String),
    Const(bool),
}

impl Expr {
    pub fn get_symbols(&self) -> HashSet<String> {
        fn set_from(s: String) -> HashSet<String> {
            let mut set = HashSet::new();
            set.insert(s);
            set
        }

        fn union(mut a: HashSet<String>, b: HashSet<String>) -> HashSet<String> {
            b.into_iter().for_each(|elem| {
                a.insert(elem);
            });
            a
        }

        use Expr::*;
        match self {
            Const(_) => HashSet::new(),
            Var(s) => set_from(s.to_string()),
            And(a, b) => union(a.get_symbols(), b.get_symbols()),
            Or(a, b) => union(a.get_symbols(), b.get_symbols()),
            Impl(a, b) => union(a.get_symbols(), b.get_symbols()),
            Biimpl(a, b) => union(a.get_symbols(), b.get_symbols()),
            Not(e) => e.get_symbols(),
        }
    }
}
