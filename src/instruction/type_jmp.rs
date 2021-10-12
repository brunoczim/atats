use crate::{
    addrmode::{Absolute, Indirect},
    instruction::GenericInstr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Jmp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Ind(Indirect),
    Abs(Absolute),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
