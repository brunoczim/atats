use crate::{
    addrmode::{Absolute, AbsoluteX, Zeropage, ZeropageX},
    instruction::GenericInstr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Inc,
    Dec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Zeropage(Zeropage),
    ZeropageX(ZeropageX),
    Absolute(Absolute),
    AbsoluteX(AbsoluteX),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
