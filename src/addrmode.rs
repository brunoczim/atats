use crate::{
    arch::{Address, RelativeAddr, Word, PageAddr},
    binary::{Decode, Decoder, NoConfig},
};
use std::io::{self, Read};

macro_rules! decode_for_wrapper {
    { $outer:ty { $field:ident } } => {
        impl Decode for $outer {
            type Config = NoConfig;

            fn decode<R>(
                _config: &Self::Config,
                decoder: &mut Decoder<R>,
            ) -> io::Result<Self>
            where
                R: Read,
            {
                let $field = decoder.decode()?;
                Ok(Self { $field })
            }
        }
    };
}

macro_rules! decode_for_unit {
    { $outer:ty } => {
        impl Decode for $outer {
            type Config = NoConfig;

            fn decode<R>(
                _config: &Self::Config,
                _decoder: &mut Decoder<R>,
            ) -> io::Result<Self>
            where
                R: Read,
            {
                Ok(Self)
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
    pub address: PageAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndirectY {
    pub address: PageAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Relative {
    pub address: RelativeAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Zeropage {
    pub address: PageAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZeropageX {
    pub address: PageAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZeropageY {
    pub address: PageAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Accumulator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Implied;

decode_for_wrapper! { Absolute { address } }
decode_for_wrapper! { AbsoluteX { address } }
decode_for_wrapper! { AbsoluteY { address } }
decode_for_wrapper! { Immediate { data } }
decode_for_wrapper! { Indirect { address } }
decode_for_wrapper! { XIndirect { address } }
decode_for_wrapper! { IndirectY { address } }
decode_for_wrapper! { Relative { address } }
decode_for_wrapper! { Zeropage { address } }
decode_for_wrapper! { ZeropageX { address } }
decode_for_wrapper! { ZeropageY { address } }
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
