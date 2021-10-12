use crate::{
    addrmode::{Absolute, Implied},
    instruction::GenericInstr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Jsr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Absolute(Absolute),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
