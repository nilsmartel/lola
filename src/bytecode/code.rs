/// Bytecode for use in Stack machine
/// only supports logic operations
#[derive(Copy, Clone, Debug)]
pub enum Code {
    And,
    Or,
    Impl,
    Biimpl,
    Not,
    Put(bool),
    Load(u8),
}
