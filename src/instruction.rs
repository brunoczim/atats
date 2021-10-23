use crate::addrmode::{AddrMode, Operand};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Aop,
    Ldx,
    Ldy,
    Sta,
    Stx,
    Sty,
    Cxy,
    Rsh,
    Idc,
    Bit,
    Jmp,
    Jsr,
    Bch,
    Imp,
}

impl Type {
    pub fn addrmode(self, opcode: u8) -> AddrMode {
        let mode_bits = (opcode >> 2) & 0x7;
        match self {
            Type::Aop => match mode_bits {
                0x0 => AddrMode::Ind,
                0x1 => AddrMode::Zpg,
                0x2 => AddrMode::Imm,
                0x3 => AddrMode::Abs,
                0x4 => AddrMode::IndY,
                0x5 => AddrMode::ZpgX,
                0x6 => AddrMode::AbsY,
                0x7 => AddrMode::AbsX,
            },
            Type::Ldx => (),
            Type::Ldy => (),
            Type::Sta => (),
            Type::Stx => (),
            Type::Sty => (),
            Type::Cxy => (),
            Type::Rsh => (),
            Type::Idc => (),
            Type::Bit => (),
            Type::Jmp => (),
            Type::Jsr => (),
            Type::Bch => (),
            Type::Imp => (),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Opcode {
    pub mnemonic: Mnemonic,
    pub addrmode: AddrMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operand: Operand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mnemonic {
    Ora,
    And,
    Eor,
    Adc,
    Lda,
    Cmp,
    Sbc,
    Bpl,
    Bmi,
    Bvc,
    Bcc,
    Bcs,
    Bne,
    Beq,
    Bit,
    Cpx,
    Cpy,
    Inc,
    Dec,
    Inx,
    Iny,
    Dex,
    Dey,
    Brk,
    Php,
    Rti,
    Clc,
    Plp,
    Sec,
    Pha,
    Cli,
    Pla,
    Sei,
    Tya,
    Tay,
    Clv,
    Cld,
    Sed,
    Nop,
    Jmp,
    Jsr,
    Ldx,
    Ldy,
    Asl,
    Rol,
    Lsr,
    Ror,
    Sta,
    Stx,
    Sty,
}
