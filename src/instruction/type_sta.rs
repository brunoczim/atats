use crate::{
    addrmode::{
        Absolute,
        AbsoluteX,
        AbsoluteY,
        IndirectY,
        XIndirect,
        Zeropage,
        ZeropageX,
    },
    instruction::Instruction as GenericInstr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Sta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    XInd(XIndirect),
    Zpg(Zeropage),
    Abs(Absolute),
    IndY(IndirectY),
    ZpgX(ZeropageX),
    AbsY(AbsoluteY),
    AbsX(AbsoluteX),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
