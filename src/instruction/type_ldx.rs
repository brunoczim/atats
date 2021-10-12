use crate::{
    addrmode::{Absolute, AbsoluteY, Immediate, Zeropage, ZeropageY},
    instruction::Instruction as GenericInstr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Ldx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Imm(Immediate),
    Zpg(Zeropage),
    ZpgY(ZeropageY),
    Abs(Absolute),
    AbsY(AbsoluteY),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
