use crate::{
    addrmode::{Absolute, AbsoluteX, Implied, Zeropage, ZeropageX},
    instruction::Instruction as GenericInstr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Asl,
    Rol,
    Lsr,
    Ror,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Zpg(Zeropage),
    ZpgX(ZeropageX),
    Impl(Implied),
    Abs(Absolute),
    AbsX(AbsoluteX),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
