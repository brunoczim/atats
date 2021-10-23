use std::{error::Error, fmt, io};

#[derive(Debug, Clone)]
pub struct BankError {
    pub bank: u8,
}

impl fmt::Display for BankError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "invalid ROM bank 0x{:x}", self.bank)
    }
}

impl Error for BankError {}

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
pub enum NoExternalError {}

#[derive(Debug, Clone)]
pub enum MachineError {
    Read(ReadError),
    Write(WriteError),
    Bank(BankError),
    Opcode(OpcodeError),
}

impl fmt::Display for MachineError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MachineError::Read(error) => write!(fmtr, "{}", error),
            MachineError::Write(error) => write!(fmtr, "{}", error),
            MachineError::Bank(error) => write!(fmtr, "{}", error),
            MachineError::Opcode(error) => write!(fmtr, "{}", error),
        }
    }
}

impl Error for MachineError {}

impl From<ReadError> for MachineError {
    fn from(error: ReadError) -> Self {
        MachineError::Read(error)
    }
}

impl From<WriteError> for MachineError {
    fn from(error: WriteError) -> Self {
        MachineError::Write(error)
    }
}

impl From<BankError> for MachineError {
    fn from(error: BankError) -> Self {
        MachineError::Bank(error)
    }
}

impl From<OpcodeError> for MachineError {
    fn from(error: OpcodeError) -> Self {
        MachineError::Opcode(error)
    }
}

impl From<MachineError> for io::Error {
    fn from(error: MachineError) -> Self {
        let kind = match error {
            MachineError::Read(_) | MachineError::Write(_) => {
                io::ErrorKind::AddrNotAvailable
            },
            MachineError::Bank(_) => io::ErrorKind::NotFound,
            MachineError::Opcode(_) => io::ErrorKind::InvalidData,
        };

        io::Error::new(kind, error)
    }
}
