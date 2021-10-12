use crate::{
    addrmode::{Absolute, Immediate, Zeropage},
    instruction::Instruction as GenericInstr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Cpx,
    Cpy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Imm(Immediate),
    Zpg(Zeropage),
    Abs(Absolute),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
