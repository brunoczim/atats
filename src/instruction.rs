use crate::{
    addrmode::{AddrMode, Operand},
    error::OpcodeError,
};

pub fn opcode_bits_a(opcode_bits: u8) -> u8 {
    opcode_bits >> 5
}

pub fn opcode_bits_b(opcode_bits: u8) -> u8 {
    (opcode_bits >> 2) & 0x7
}

pub fn opcode_bits_c(opcode_bits: u8) -> u8 {
    opcode_bits & 0x3
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

impl Type {
    pub fn addrmode(self, opcode: u8) -> Result<AddrMode, OpcodeError> {
        let mode_bits = opcode_bits_b(opcode);
        let jmp_mode_bits = opcode_bits_a(opcode);
        match self {
            Type::Aop => match mode_bits {
                0x0 => Ok(AddrMode::Ind),
                0x1 => Ok(AddrMode::Zpg),
                0x2 => Ok(AddrMode::Imm),
                0x3 => Ok(AddrMode::Abs),
                0x4 => Ok(AddrMode::IndY),
                0x5 => Ok(AddrMode::ZpgX),
                0x6 => Ok(AddrMode::AbsY),
                0x7 => Ok(AddrMode::AbsX),
                _ => Err(OpcodeError { bits: opcode }),
            },
            Type::Ldx => match mode_bits {
                0x0 => Ok(AddrMode::Imm),
                0x1 => Ok(AddrMode::Zpg),
                0x3 => Ok(AddrMode::Abs),
                0x5 => Ok(AddrMode::ZpgY),
                0x7 => Ok(AddrMode::AbsY),
                _ => Err(OpcodeError { bits: opcode }),
            },
            Type::Ldy => match mode_bits {
                0x0 => Ok(AddrMode::Imm),
                0x1 => Ok(AddrMode::Zpg),
                0x3 => Ok(AddrMode::Abs),
                0x5 => Ok(AddrMode::ZpgX),
                0x7 => Ok(AddrMode::AbsX),
                _ => Err(OpcodeError { bits: opcode }),
            },
            Type::Sta => match mode_bits {
                0x0 => Ok(AddrMode::Ind),
                0x1 => Ok(AddrMode::Zpg),
                0x3 => Ok(AddrMode::Abs),
                0x4 => Ok(AddrMode::IndY),
                0x5 => Ok(AddrMode::ZpgX),
                0x6 => Ok(AddrMode::AbsY),
                0x7 => Ok(AddrMode::AbsX),
                _ => Err(OpcodeError { bits: opcode }),
            },
            Type::Stx => match mode_bits {
                0x1 => Ok(AddrMode::Zpg),
                0x3 => Ok(AddrMode::Abs),
                0x5 => Ok(AddrMode::ZpgY),
                _ => Err(OpcodeError { bits: opcode }),
            },
            Type::Sty => match mode_bits {
                0x1 => Ok(AddrMode::Zpg),
                0x3 => Ok(AddrMode::Abs),
                0x5 => Ok(AddrMode::ZpgX),
                _ => Err(OpcodeError { bits: opcode }),
            },
            Type::Cxy => match mode_bits {
                0x0 => Ok(AddrMode::Imm),
                0x1 => Ok(AddrMode::Zpg),
                _ => Err(OpcodeError { bits: opcode }),
            },
            Type::Rsh => match mode_bits {
                0x1 => Ok(AddrMode::Zpg),
                0x2 => Ok(AddrMode::Acc),
                0x3 => Ok(AddrMode::Abs),
                0x5 => Ok(AddrMode::ZpgX),
                0x7 => Ok(AddrMode::AbsX),
                _ => Err(OpcodeError { bits: opcode }),
            },
            Type::Idc => match mode_bits {
                0x1 => Ok(AddrMode::Zpg),
                0x3 => Ok(AddrMode::Abs),
                0x5 => Ok(AddrMode::ZpgX),
                0x7 => Ok(AddrMode::AbsX),
                _ => Err(OpcodeError { bits: opcode }),
            },
            Type::Bit => match mode_bits {
                0x1 => Ok(AddrMode::Zpg),
                0x3 => Ok(AddrMode::Abs),
                _ => Err(OpcodeError { bits: opcode }),
            },
            Type::Jmp => match jmp_mode_bits {
                0x2 => Ok(AddrMode::Abs),
                0x3 => Ok(AddrMode::Ind),
                _ => Err(OpcodeError { bits: opcode }),
            },
            Type::Jsr => Ok(AddrMode::Abs),
            Type::Bch => Ok(AddrMode::Rel),
            Type::Imp => Ok(AddrMode::Impl),
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
