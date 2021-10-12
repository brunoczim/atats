use crate::arch::{Address, RelativeAddr, Word, ZeropageAddr};
use nom::{
    combinator::{map, success},
    IResult,
};

macro_rules! decode_for_wrapper {
    { $ty:ty { $field:ident: $field_ty:ty } } => {
        impl $ty {
            pub fn decode(input: &[u8]) -> IResult<&[u8], Self> {
                map(<$field_ty>::decode, |$field| Self { $field })(input)
            }
        }
    };
}

macro_rules! decode_for_unit {
    { $ty:ty } => {
        impl $ty {
            pub fn decode(input: &[u8]) -> IResult<&[u8], Self> {
                success(Self)(input)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Absolute {
    pub address: Address,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AbsoluteX {
    pub address: Address,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AbsoluteY {
    pub address: Address,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Immediate {
    pub data: Word,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Indirect {
    pub address: Address,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct XIndirect {
    pub address: ZeropageAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndirectY {
    pub address: ZeropageAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Relative {
    pub address: RelativeAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Zeropage {
    pub address: ZeropageAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZeropageX {
    pub address: ZeropageAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZeropageY {
    pub address: ZeropageAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Accumulator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Implied;

decode_for_wrapper! { Absolute { address: Address } }
decode_for_wrapper! { AbsoluteX { address: Address } }
decode_for_wrapper! { AbsoluteY { address: Address } }
decode_for_wrapper! { Immediate { data: Word } }
decode_for_wrapper! { Indirect { address: Address } }
decode_for_wrapper! { XIndirect { address: ZeropageAddr } }
decode_for_wrapper! { Relative { address: RelativeAddr } }
decode_for_wrapper! { Zeropage { address: ZeropageAddr } }
decode_for_wrapper! { ZeropageX { address: ZeropageAddr } }
decode_for_wrapper! { ZeropageY { address: ZeropageAddr } }
decode_for_unit! { Accumulator }
decode_for_unit! { Implied }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AOperAddrMode {
    XInd(XIndirect),
    Zpg(Zeropage),
    Imm(Immediate),
    Abs(Address),
    IndY(IndirectY),
    ZpgX(ZeropageX),
    AbsY(AbsoluteY),
    AbsX(AbsoluteX),
}

impl AOperAddrMode {
    pub const CODE_X_IND: u8 = 0;
    pub const CODE_ZPG: u8 = 1;
    pub const CODE_IMM: u8 = 2;
    pub const CODE_ABS: u8 = 3;

    pub const CODE_IND_Y: u8 = 4;
    pub const CODE_ZPG_X: u8 = 5;
    pub const CODE_ABS_Y: u8 = 6;
    pub const CODE_ABS_X: u8 = 7;
}
