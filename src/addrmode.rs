use crate::binary::{Decode, Decoder, NoConfig};
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
    pub address: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AbsoluteX {
    pub address: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AbsoluteY {
    pub address: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Immediate {
    pub bits: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Indirect {
    pub address: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct XIndirect {
    pub address: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndirectY {
    pub address: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Relative {
    pub address: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Zeropage {
    pub address: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZeropageX {
    pub address: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZeropageY {
    pub address: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Accumulator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Implied;

decode_for_wrapper! { Absolute { address } }
decode_for_wrapper! { AbsoluteX { address } }
decode_for_wrapper! { AbsoluteY { address } }
decode_for_wrapper! { Immediate { bits } }
decode_for_wrapper! { Indirect { address } }
decode_for_wrapper! { XIndirect { address } }
decode_for_wrapper! { IndirectY { address } }
decode_for_wrapper! { Relative { address } }
decode_for_wrapper! { Zeropage { address } }
decode_for_wrapper! { ZeropageX { address } }
decode_for_wrapper! { ZeropageY { address } }
decode_for_unit! { Accumulator }
decode_for_unit! { Implied }
