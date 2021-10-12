use crate::{
    addrmode::{Absolute, Zeropage},
    instruction::Instruction as GenericInstr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Bit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Zpg(Zeropage),
    Abs(Absolute),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
