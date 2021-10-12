use crate::{addrmode::Relative, instruction::GenericInstr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Bpl,
    Bmi,
    Bvc,
    Bcc,
    Bcs,
    Bne,
    Beq,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Relative(Relative),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
