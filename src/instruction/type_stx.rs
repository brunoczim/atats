use crate::{
    addrmode::{Absolute, Zeropage, ZeropageY},
    instruction::GenericInstr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Stx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Zpg(Zeropage),
    Abs(Absolute),
    ZpgY(ZeropageY),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
