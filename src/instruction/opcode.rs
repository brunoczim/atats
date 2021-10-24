use crate::{
    addrmode::AddrMode,
    binary::{Decode, Decoder, Encode, Encoder, NoConfig},
    error::{AddrModeError, MachineError, OpcodeError},
    instruction::mnemonic::Mnemonic,
};

pub fn bits_a(opcode_bits: u8) -> u8 {
    opcode_bits >> 5
}

pub fn bits_b(opcode_bits: u8) -> u8 {
    (opcode_bits >> 2) & 0x7
}

pub fn bits_c(opcode_bits: u8) -> u8 {
    opcode_bits & 0x3
}

pub fn set_bits_a(opcode_bits: u8, value: u8) -> u8 {
    (opcode_bits & 0x1F) | (value << 5)
}

pub fn set_bits_b(opcode_bits: u8, value: u8) -> u8 {
    (opcode_bits & 0xE3) | ((value & 0x7) << 2)
}

pub fn set_bits_c(opcode_bits: u8, value: u8) -> u8 {
    (opcode_bits & 0xF8) | (value & 0x3)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Opcode {
    pub mnemonic: Mnemonic,
    pub addrmode: AddrMode,
}

impl Opcode {
    pub fn from_bits(bits: u8) -> Result<Self, OpcodeError> {
        let mnemonic = Mnemonic::from_opcode_bits(bits)?;
        let addrmode = mnemonic.instr_type().addrmode_from_bits(bits)?;
        Ok(Self { mnemonic, addrmode })
    }

    pub fn to_bits(self) -> Result<u8, AddrModeError> {
        let mnemonic = self.mnemonic.to_opcode_bits();
        let instr_type = self.mnemonic.instr_type();
        let addrmode = instr_type.addrmode_to_bits(self.addrmode)?;
        Ok(mnemonic | addrmode)
    }
}

impl Encode for Opcode {
    fn encode<E>(&self, encoder: &mut E) -> Result<(), E::Error>
    where
        E: Encoder + ?Sized,
    {
        let bits = self.to_bits().map_err(MachineError::from)?;
        encoder.encode(bits)
    }
}

impl Decode for Opcode {
    type Config = NoConfig;

    fn decode<D>(
        _config: &Self::Config,
        decoder: &mut D,
    ) -> Result<Self, D::Error>
    where
        D: Decoder + ?Sized,
    {
        let bits = decoder.decode()?;
        Self::from_bits(bits).map_err(MachineError::from).map_err(Into::into)
    }
}
