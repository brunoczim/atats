use crate::binary::{Decode, Decoder, NoConfig};
use std::io::{self, Read};

pub type WordBits = u8;
pub type AddressBits = u16;
pub type PageAddrBits = u8;
pub type RelativeAddrBits = i8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Word {
    pub bits: WordBits,
}

impl Decode for Word {
    type Config = NoConfig;

    fn decode<R>(
        _config: &Self::Config,
        decoder: &mut Decoder<R>,
    ) -> io::Result<Self>
    where
        R: Read,
    {
        let mut buf = [0; 1];
        decoder.read_exact(&mut buf)?;
        Ok(Self { bits: buf[0] })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Address {
    pub bits: AddressBits,
}

impl Decode for Address {
    type Config = NoConfig;

    fn decode<R>(
        _config: &Self::Config,
        decoder: &mut Decoder<R>,
    ) -> io::Result<Self>
    where
        R: Read,
    {
        let mut buf = [0; 2];
        decoder.read_exact(&mut buf)?;
        Ok(Self { bits: AddressBits::from_le_bytes(buf) })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PageAddr {
    pub bits: PageAddrBits,
}

impl Decode for PageAddr {
    type Config = NoConfig;

    fn decode<R>(
        _config: &Self::Config,
        decoder: &mut Decoder<R>,
    ) -> io::Result<Self>
    where
        R: Read,
    {
        let mut buf = [0; 1];
        decoder.read_exact(&mut buf)?;
        Ok(Self { bits: buf[0] })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RelativeAddr {
    pub bits: RelativeAddrBits,
}

impl Decode for RelativeAddr {
    type Config = NoConfig;

    fn decode<R>(
        _config: &Self::Config,
        decoder: &mut Decoder<R>,
    ) -> io::Result<Self>
    where
        R: Read,
    {
        let mut buf = [0; 1];
        decoder.read_exact(&mut buf)?;
        Ok(Self { bits: RelativeAddrBits::from_le_bytes(buf) })
    }
}
