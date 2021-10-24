use crate::{
    addrmode::{AddrMode, Operand},
    error::{AddrModeError, OpcodeError},
};
use std::fmt;

pub fn opcode_bits_a(opcode_bits: u8) -> u8 {
    opcode_bits >> 5
}

pub fn opcode_bits_b(opcode_bits: u8) -> u8 {
    (opcode_bits >> 2) & 0x7
}

pub fn opcode_bits_c(opcode_bits: u8) -> u8 {
    opcode_bits & 0x3
}

pub fn set_opcode_bits_a(opcode_bits: u8, value: u8) -> u8 {
    (opcode_bits & 0x1F) | (value << 5)
}

pub fn set_opcode_bits_b(opcode_bits: u8, value: u8) -> u8 {
    (opcode_bits & 0xE3) | ((value & 0x7) << 2)
}

pub fn set_opcode_bits_c(opcode_bits: u8, value: u8) -> u8 {
    (opcode_bits & 0xF8) | (value & 0x3)
}

#[allow(unused_macros)]
macro_rules! types_addrmode {
    (
        match instr_type {
            $($variant:ident => match $bits:ident { $($tok:tt)* },)*
        }
    ) => {
        impl Type {
            pub fn addrmode(self, opcode: u8) -> Result<AddrMode, OpcodeError> {
                match self {
                    $(Type::$variant => type_to_addrmode!(
                        opcode,
                        $bits,
                        $($tok)*
                    )),*
                }
            }

            pub fn opcode_bits(
                self,
                addrmode: AddrMode
            ) -> Result<u8, AddrModeError> {
                match self {
                    $(Type::$variant => type_to_opcode!(
                        addrmode,
                        self,
                        $bits,
                        $($tok)*
                    )),*
                }
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! type_to_opcode {
    (
        $mode:expr,
        $instr_type:expr,
        mode_bits_a,
        $($opcode_pat:literal => $mode_pat:ident,)*
    ) => {
        match $mode {
            $(AddrMode::$mode_pat => Ok(set_opcode_bits_a(0, $opcode_pat)),)*
            _ => Err(AddrModeError { mode: $mode, instr_type: $instr_type }),
        }
    };

    (
        $mode:expr,
        $instr_type:expr,
        mode_bits_b,
        $($opcode_pat:literal => $mode_pat:ident,)*
    ) => {
        match $mode {
            $(AddrMode::$mode_pat => Ok(set_opcode_bits_b(0, $opcode_pat)),)*
            _ => Err(AddrModeError { mode: $mode, instr_type: $instr_type }),
        }
    };

    (
        $mode:expr,
        $instr_type:expr,
        no_mode_bits,
        _ => $mode_pat:ident,
    ) => {
        match $mode {
            AddrMode::$mode_pat => Ok(0),
            _ => Err(AddrModeError { mode: $mode, instr_type: $instr_type }),
        }
    };
}

#[allow(unused_macros)]
macro_rules! type_to_addrmode {
    (
        $opcode:expr,
        mode_bits_a,
        $($opcode_pat:literal => $mode_pat:ident,)*
    ) => {
        match opcode_bits_a($opcode) {
            $($opcode_pat => Ok(AddrMode::$mode_pat),)*
            _ => Err(OpcodeError { bits: $opcode }),
        }
    };

    (
        $opcode:expr,
        mode_bits_b,
        $($opcode_pat:literal => $mode_pat:ident,)*
    ) => {
        match opcode_bits_b($opcode) {
            $($opcode_pat => Ok(AddrMode::$mode_pat),)*
            _ => Err(OpcodeError { bits: $opcode }),
        }
    };

    (
        $opcode:expr,
        no_mode_bits,
        _ => $mode_pat:ident,
    ) => {
        Ok(AddrMode::$mode_pat)
    };
}

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

impl fmt::Display for Type {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Aop => write!(fmtr, "AOP"),
            Type::Ldx => write!(fmtr, "LDX"),
            Type::Ldy => write!(fmtr, "LDY"),
            Type::Sta => write!(fmtr, "STA"),
            Type::Stx => write!(fmtr, "STX"),
            Type::Sty => write!(fmtr, "STY"),
            Type::Cxy => write!(fmtr, "CXY"),
            Type::Rsh => write!(fmtr, "RSH"),
            Type::Idc => write!(fmtr, "IDC"),
            Type::Bit => write!(fmtr, "BIT"),
            Type::Jmp => write!(fmtr, "JMP"),
            Type::Jsr => write!(fmtr, "JSR"),
            Type::Bch => write!(fmtr, "BCH"),
            Type::Imp => write!(fmtr, "IMP"),
        }
    }
}

types_addrmode! {
    match instr_type {
        Aop => match mode_bits_b {
            0x0 => Ind,
            0x1 => Zpg,
            0x2 => Imm,
            0x3 => Abs,
            0x4 => IndY,
            0x5 => ZpgX,
            0x6 => AbsY,
            0x7 => AbsX,
        },
        Ldx => match mode_bits_b {
            0x0 => Imm,
            0x1 => Zpg,
            0x3 => Abs,
            0x5 => ZpgY,
            0x7 => AbsY,
        },
        Ldy => match mode_bits_b {
            0x0 => Imm,
            0x1 => Zpg,
            0x3 => Abs,
            0x5 => ZpgX,
            0x7 => AbsX,
        },
        Sta => match mode_bits_b {
            0x0 => Ind,
            0x1 => Zpg,
            0x3 => Abs,
            0x4 => IndY,
            0x5 => ZpgX,
            0x6 => AbsY,
            0x7 => AbsX,
        },
        Stx => match mode_bits_b {
            0x1 => Zpg,
            0x3 => Abs,
            0x5 => ZpgY,
        },
        Sty => match mode_bits_b {
            0x1 => Zpg,
            0x3 => Abs,
            0x5 => ZpgX,
        },
        Cxy => match mode_bits_b {
            0x0 => Imm,
            0x1 => Zpg,
        },
        Rsh => match mode_bits_b {
            0x1 => Zpg,
            0x2 => Acc,
            0x3 => Abs,
            0x5 => ZpgX,
            0x7 => AbsX,
        },
        Idc => match mode_bits_b {
            0x1 => Zpg,
            0x3 => Abs,
            0x5 => ZpgX,
            0x7 => AbsX,
        },
        Bit => match mode_bits_b {
            0x1 => Zpg,
            0x3 => Abs,
        },
        Jmp => match mode_bits_a {
            0x2 => Abs,
            0x3 => Ind,
        },
        Jsr => match no_mode_bits {
            _ => Abs,
        },
        Bch => match no_mode_bits {
            _ => Rel,
        },
        Imp => match no_mode_bits {
            _ => Impl,
        },
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
    Bvs,
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
    Rts,
    Clc,
    Plp,
    Sec,
    Pha,
    Cli,
    Pla,
    Sei,
    Tya,
    Tay,
    Txa,
    Txs,
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

impl Mnemonic {
    pub fn from_opcode_bits(opcode: u8) -> Result<Self, OpcodeError> {
        let bits_a = opcode_bits_a(opcode);
        let bits_b = opcode_bits_b(opcode);
        let bits_c = opcode_bits_c(opcode);
        match (bits_a, bits_b, bits_c) {
            (0, 0, 0) => Ok(Mnemonic::Brk),
            (0, 2, 0) => Ok(Mnemonic::Php),
            (0, 4, 0) => Ok(Mnemonic::Bpl),
            (0, 6, 0) => Ok(Mnemonic::Clc),
            (0, _, 1) => Ok(Mnemonic::Ora),
            (0, _, 2) => Ok(Mnemonic::Asl),
            (1, 0, 0) => Ok(Mnemonic::Jsr),
            (1, 1 | 3, 0) => Ok(Mnemonic::Bit),
            (1, 2, 0) => Ok(Mnemonic::Plp),
            (1, 4, 0) => Ok(Mnemonic::Bmi),
            (1, 6, 0) => Ok(Mnemonic::Sec),
            (1, _, 1) => Ok(Mnemonic::And),
            (1, _, 2) => Ok(Mnemonic::Rol),
            (2, 0, 0) => Ok(Mnemonic::Rts),
            (2, 2, 0) => Ok(Mnemonic::Pha),
            (2 | 3, 3, 0) => Ok(Mnemonic::Jmp),
            (2, 4, 0) => Ok(Mnemonic::Bvc),
            (2, 6, 0) => Ok(Mnemonic::Cli),
            (2, _, 1) => Ok(Mnemonic::Eor),
            (2, _, 2) => Ok(Mnemonic::Lsr),
            (3, 0, 0) => Ok(Mnemonic::Rts),
            (3, 2, 0) => Ok(Mnemonic::Pla),
            (3, 4, 0) => Ok(Mnemonic::Bvs),
            (3, 6, 0) => Ok(Mnemonic::Sei),
            (3, _, 0) => Ok(Mnemonic::Adc),
            (3, _, 1) => Ok(Mnemonic::Ror),
            (4, 1 | 3 | 5, 0) => Ok(Mnemonic::Sty),
            (4, 2, 0) => Ok(Mnemonic::Dey),
            (4, 4, 0) => Ok(Mnemonic::Bcc),
            (4, 6, 0) => Ok(Mnemonic::Tya),
            (4, _, 1) => Ok(Mnemonic::Sta),
            (4, 1 | 3 | 5, 2) => Ok(Mnemonic::Stx),
            (4, 2, 2) => Ok(Mnemonic::Txa),
            (4, 6, 2) => Ok(Mnemonic::Txs),
            (5, 0 | 1 | 3 | 5 | 7, 0) => Ok(Mnemonic::Ldy),
            (5, 2, 0) => Ok(Mnemonic::Tay),
            (5, 4, 0) => Ok(Mnemonic::Bcs),
            (5, 6, 0) => Ok(Mnemonic::Clv),
            (5, _, 1) => Ok(Mnemonic::Lda),
            (5, 0 | 1 | 3 | 5 | 7, 2) => Ok(Mnemonic::Ldx),
            (6, 0 | 1 | 3, 0) => Ok(Mnemonic::Cpy),
            (6, 2, 0) => Ok(Mnemonic::Iny),
            (6, 4, 0) => Ok(Mnemonic::Bne),
            (6, 6, 0) => Ok(Mnemonic::Cld),
            (6, _, 1) => Ok(Mnemonic::Cmp),
            (6, 1 | 3 | 5 | 7, 2) => Ok(Mnemonic::Dec),
            (6, 2, 2) => Ok(Mnemonic::Dex),
            (7, 0 | 1 | 3, 0) => Ok(Mnemonic::Cpx),
            (7, 2, 0) => Ok(Mnemonic::Inx),
            (7, 4, 0) => Ok(Mnemonic::Beq),
            (7, 6, 0) => Ok(Mnemonic::Sed),
            (7, _, 1) => Ok(Mnemonic::Sbc),
            (7, 1 | 3 | 5 | 7, 2) => Ok(Mnemonic::Inc),
            (7, 2, 2) => Ok(Mnemonic::Nop),
            _ => Err(OpcodeError { bits: opcode }),
        }
    }

    pub fn instr_type(self) -> Type {
        match self {
            Mnemonic::Ora
            | Mnemonic::And
            | Mnemonic::Eor
            | Mnemonic::Adc
            | Mnemonic::Lda
            | Mnemonic::Cmp
            | Mnemonic::Sbc => Type::Aop,
            Mnemonic::Bpl
            | Mnemonic::Bmi
            | Mnemonic::Bvc
            | Mnemonic::Bvs
            | Mnemonic::Bcc
            | Mnemonic::Bcs
            | Mnemonic::Bne
            | Mnemonic::Beq => Type::Bch,
            Mnemonic::Bit => Type::Bit,
            Mnemonic::Cpx | Mnemonic::Cpy => Type::Cxy,
            Mnemonic::Inc | Mnemonic::Dec => Type::Idc,
            Mnemonic::Inx
            | Mnemonic::Iny
            | Mnemonic::Dex
            | Mnemonic::Dey
            | Mnemonic::Brk
            | Mnemonic::Php
            | Mnemonic::Rti
            | Mnemonic::Rts
            | Mnemonic::Clc
            | Mnemonic::Plp
            | Mnemonic::Sec
            | Mnemonic::Pha
            | Mnemonic::Cli
            | Mnemonic::Pla
            | Mnemonic::Sei
            | Mnemonic::Txa
            | Mnemonic::Txs
            | Mnemonic::Tya
            | Mnemonic::Tay
            | Mnemonic::Clv
            | Mnemonic::Cld
            | Mnemonic::Sed
            | Mnemonic::Nop => Type::Imp,
            Mnemonic::Jmp => Type::Jmp,
            Mnemonic::Jsr => Type::Jsr,
            Mnemonic::Ldx => Type::Ldx,
            Mnemonic::Ldy => Type::Ldy,
            Mnemonic::Asl | Mnemonic::Rol | Mnemonic::Lsr | Mnemonic::Ror => {
                Type::Rsh
            },
            Mnemonic::Sta => Type::Sta,
            Mnemonic::Stx => Type::Stx,
            Mnemonic::Sty => Type::Sty,
        }
    }
}
