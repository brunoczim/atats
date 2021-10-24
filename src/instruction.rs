pub mod opcode;
mod itype;
mod mnemonic;

pub use itype::Type;
pub use mnemonic::Mnemonic;
pub use opcode::Opcode;

use crate::{
    addrmode::Operand,
    binary::{Decode, Decoder, Encode, Encoder, NoConfig},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operand: Operand,
}

impl Instruction {
    pub fn opcode(self) -> Opcode {
        Opcode { mnemonic: self.mnemonic, addrmode: self.operand.addrmode() }
    }
}

impl Encode for Instruction {
    fn encode<E>(&self, encoder: &mut E) -> Result<(), E::Error>
    where
        E: Encoder + ?Sized,
    {
        encoder.encode(self.opcode())?;
        encoder.encode(self.operand)?;
        Ok(())
    }
}

impl Decode for Instruction {
    type Config = NoConfig;

    fn decode<D>(
        _config: &Self::Config,
        decoder: &mut D,
    ) -> Result<Self, D::Error>
    where
        D: Decoder + ?Sized,
    {
        let opcode = decoder.decode::<Opcode>()?;
        let operand = decoder.decode_with(&opcode.addrmode)?;
        Ok(Self { mnemonic: opcode.mnemonic, operand })
    }
}
