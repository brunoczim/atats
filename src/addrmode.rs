use crate::binary::{Decode, Decoder, Encode, Encoder, NoConfig};
use std::io::{self, Read, Write};

macro_rules! decode_for_wrapper {
    { $outer:ty { $field:ident: $field_ty:ty } } => {
        impl Decode for $outer {
            type Config = <$field_ty as Decode>::Config;

            fn decode<R>(
                config: &Self::Config,
                decoder: &mut Decoder<R>,
            ) -> io::Result<Self>
            where
                R: Read,
            {
                let $field = decoder.decode_with(config)?;
                Ok(Self { $field })
            }
        }

        impl Encode for $outer {
            type Config = <$field_ty as Encode>::Config;

            fn encode<W>(
                &self,
                config: &Self::Config,
                encoder: &mut Encoder<W>,
            ) -> io::Result<()>
            where
                W: Write
            {
                encoder.encode_with(&self.$field, config)
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

        impl Encode for $outer {
            type Config = NoConfig;

            fn encode<W>(
                &self,
                _config: &Self::Config,
                _encoder: &mut Encoder<W>,
            ) -> io::Result<()>
            where
                W: Write
            {
                Ok(())
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

decode_for_wrapper! { Absolute { address: u8 } }
decode_for_wrapper! { AbsoluteX { address: u8 } }
decode_for_wrapper! { AbsoluteY { address: u8 } }
decode_for_wrapper! { Immediate { bits: u8 } }
decode_for_wrapper! { Indirect { address: u16 } }
decode_for_wrapper! { XIndirect { address: u8 } }
decode_for_wrapper! { IndirectY { address: u8 } }
decode_for_wrapper! { Relative { address: i8 } }
decode_for_wrapper! { Zeropage { address: u8 } }
decode_for_wrapper! { ZeropageX { address: u8 } }
decode_for_wrapper! { ZeropageY { address: u8 } }
decode_for_unit! { Accumulator }
decode_for_unit! { Implied }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddrMode {
    Acc,
    Abs,
    AbsX,
    AbsY,
    Imm,
    Impl,
    Ind,
    XInd,
    IndY,
    Rel,
    Zpg,
    ZpgX,
    ZpgY,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operand {
    Acc(Accumulator),
    Abs(Absolute),
    AbsX(AbsoluteX),
    AbsY(AbsoluteY),
    Imm(Immediate),
    Impl(Implied),
    Ind(Indirect),
    XInd(XIndirect),
    IndY(IndirectY),
    Rel(Relative),
    Zpg(Zeropage),
    ZpgX(ZeropageX),
    ZpgY(ZeropageY),
}

impl Operand {
    pub fn addrmode(self) -> AddrMode {
        match self {
            Operand::Acc(_) => AddrMode::Acc,
            Operand::Abs(_) => AddrMode::Abs,
            Operand::AbsX(_) => AddrMode::AbsX,
            Operand::AbsY(_) => AddrMode::AbsY,
            Operand::Imm(_) => AddrMode::Imm,
            Operand::Impl(_) => AddrMode::Impl,
            Operand::Ind(_) => AddrMode::Ind,
            Operand::XInd(_) => AddrMode::XInd,
            Operand::IndY(_) => AddrMode::IndY,
            Operand::Rel(_) => AddrMode::Rel,
            Operand::Zpg(_) => AddrMode::Zpg,
            Operand::ZpgX(_) => AddrMode::ZpgX,
            Operand::ZpgY(_) => AddrMode::ZpgY,
        }
    }
}

impl Decode for Operand {
    type Config = AddrMode;

    fn decode<R>(
        config: &Self::Config,
        decoder: &mut Decoder<R>,
    ) -> io::Result<Self>
    where
        R: Read,
    {
        match config {
            AddrMode::Acc => decoder.decode().map(Operand::Acc),
            AddrMode::Abs => decoder.decode().map(Operand::Abs),
            AddrMode::AbsX => decoder.decode().map(Operand::AbsX),
            AddrMode::AbsY => decoder.decode().map(Operand::AbsY),
            AddrMode::Imm => decoder.decode().map(Operand::Imm),
            AddrMode::Impl => decoder.decode().map(Operand::Impl),
            AddrMode::Ind => decoder.decode().map(Operand::Ind),
            AddrMode::XInd => decoder.decode().map(Operand::XInd),
            AddrMode::IndY => decoder.decode().map(Operand::IndY),
            AddrMode::Rel => decoder.decode().map(Operand::Rel),
            AddrMode::Zpg => decoder.decode().map(Operand::Zpg),
            AddrMode::ZpgX => decoder.decode().map(Operand::ZpgX),
            AddrMode::ZpgY => decoder.decode().map(Operand::ZpgY),
        }
    }
}

impl Encode for Operand {
    type Config = NoConfig;

    fn encode<W>(
        &self,
        _config: &Self::Config,
        encoder: &mut Encoder<W>,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        match self {
            Operand::Acc(data) => encoder.encode(data),
            Operand::Abs(data) => encoder.encode(data),
            Operand::AbsX(data) => encoder.encode(data),
            Operand::AbsY(data) => encoder.encode(data),
            Operand::Imm(data) => encoder.encode(data),
            Operand::Impl(data) => encoder.encode(data),
            Operand::Ind(data) => encoder.encode(data),
            Operand::XInd(data) => encoder.encode(data),
            Operand::IndY(data) => encoder.encode(data),
            Operand::Rel(data) => encoder.encode(data),
            Operand::Zpg(data) => encoder.encode(data),
            Operand::ZpgX(data) => encoder.encode(data),
            Operand::ZpgY(data) => encoder.encode(data),
        }
    }
}
