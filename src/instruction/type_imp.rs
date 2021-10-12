use crate::{addrmode::Implied, instruction::Instruction as GenericInstr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Imp(Implied),
}

pub type Instruction = GenericInstr<Opcode, AddrMode>;
