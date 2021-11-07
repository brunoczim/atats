use crate::{
    addrmode::AddrMode,
    error::{AddrModeError, OpcodeError},
    instruction::opcode,
};
use std::fmt;

macro_rules! types_addrmode {
    (
        match instr_type {
            $($variant:ident => match $bits:ident { $($tok:tt)* },)*
        }
    ) => {
        impl Type {
            pub fn addrmode_from_bits(
                self,
                opcode: u8
            ) -> Result<AddrMode, OpcodeError> {
                match self {
                    $(Type::$variant => type_to_addrmode!(
                        opcode,
                        $bits,
                        $($tok)*
                    )),*
                }
            }

            pub fn addrmode_to_bits(
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
            $(AddrMode::$mode_pat => Ok(opcode::set_bits_a(0, $opcode_pat)),)*
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
            $(AddrMode::$mode_pat => Ok(opcode::set_bits_b(0, $opcode_pat)),)*
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
        match opcode::bits_a($opcode) {
            $($opcode_pat => Ok(AddrMode::$mode_pat),)*
            _ => Err(OpcodeError { bits: $opcode }),
        }
    };

    (
        $opcode:expr,
        mode_bits_b,
        $($opcode_pat:literal => $mode_pat:ident,)*
    ) => {
        match opcode::bits_b($opcode) {
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
