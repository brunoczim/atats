use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct ReadError {
    pub address: u16,
}

impl fmt::Display for ReadError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "invalid read at address 0x{:x}", self.address)
    }
}

impl Error for ReadError {}

#[derive(Debug, Clone)]
pub struct WriteError {
    pub address: u16,
}

impl fmt::Display for WriteError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "invalid write at address 0x{:x}", self.address)
    }
}

impl Error for WriteError {}

#[derive(Debug, Clone)]
pub struct OpcodeError {
    pub bits: u8,
}

impl fmt::Display for OpcodeError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "invalid opcode 0x{:x}", self.bits)
    }
}

impl Error for OpcodeError {}

#[derive(Debug, Clone)]
pub enum MachineError {
    Read(ReadError),
    Write(WriteError),
    Opcode(OpcodeError),
}
