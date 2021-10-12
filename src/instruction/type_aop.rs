use crate::{
    addrmode::{
        Absolute,
        AbsoluteX,
        AbsoluteY,
        Immediate,
        IndirectY,
        XIndirect,
        Zeropage,
        ZeropageX,
    },
    instruction::GenericInstr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Ora,
    And,
    Eor,
    Adc,
    Lda,
    Cmp,
    Sbc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    XInd(XIndirect),
    Zpg(Zeropage),
    Imm(Immediate),
    Abs(Absolute),
    IndY(IndirectY),
    ZpgX(ZeropageX),
    AbsY(AbsoluteY),
    AbsX(AbsoluteX),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
