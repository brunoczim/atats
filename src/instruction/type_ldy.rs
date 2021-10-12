use crate::{
    addrmode::{Absolute, AbsoluteX, Immediate, Zeropage, ZeropageX},
    instruction::Instruction as GenericInstr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Ldy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Imm(Immediate),
    Zpg(Zeropage),
    ZpgX(ZeropageX),
    Abs(Absolute),
    AbsX(AbsoluteX),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
