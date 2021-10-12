use nom::{bytes::complete::take, combinator::map, IResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Word {
    pub bits: u8,
}

impl Word {
    fn from_bits(bits: &[u8]) -> Self {
        Self { bits: bits[0] }
    }

    pub fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(take(1usize), Self::from_bits)(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Address {
    pub bits: u16,
}

impl Address {
    fn from_bits(bits: &[u8]) -> Self {
        Self { bits: u16::from_le_bytes([bits[0], bits[1]]) }
    }

    pub fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(take(2usize), Self::from_bits)(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZeropageAddr {
    pub bits: u8,
}

impl ZeropageAddr {
    fn from_bits(bits: &[u8]) -> Self {
        Self { bits: bits[0] }
    }

    pub fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(take(1usize), Self::from_bits)(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RelativeAddr {
    pub bits: i8,
}

impl RelativeAddr {
    fn from_bits(bits: &[u8]) -> Self {
        Self { bits: i8::from_le_bytes([bits[0]]) }
    }

    pub fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(take(2usize), Self::from_bits)(input)
    }
}
