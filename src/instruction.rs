use crate::addrmode::{AddrMode, Operand};

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
