use crate::{
    addrmode::{Absolute, Zeropage, ZeropageX},
    instruction::Instruction as GenericInstr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Sty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Zpg(Zeropage),
    Abs(Absolute),
    ZpgX(ZeropageX),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
