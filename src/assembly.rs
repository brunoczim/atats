pub mod disassemble;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Syntax {
    Classic,
    Atats,
    Detailed,
}
