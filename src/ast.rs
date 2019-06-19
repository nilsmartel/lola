type BE = Box<Expr>;

pub enum Expr {
    And(BE, BE),
    Or(BE, BE),
    Impl(BE, BE),
    Biimpl(BE, BE),
    Not(BE, BE),
    Var(String),
    Const(bool),
}
