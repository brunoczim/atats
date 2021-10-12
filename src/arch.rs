use crate::binary::{Decode, Decoder, NoConfig};
use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Word {
    pub bits: u8,
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
    pub bits: u16,
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
        Ok(Self { bits: u16::from_le_bytes(buf) })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZeropageAddr {
    pub bits: u8,
}

impl Decode for ZeropageAddr {
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
    pub bits: i8,
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
        Ok(Self { bits: i8::from_le_bytes(buf) })
    }
}
