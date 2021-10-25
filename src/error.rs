use crate::addrmode::Operand;
use std::{error::Error, fmt, io};

use crate::{addrmode::AddrMode, instruction::Type};

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
pub struct AddrModeError {
    pub mode: AddrMode,
    pub instr_type: Type,
}

impl fmt::Display for AddrModeError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "invalid addressing mode combination of {} with instruction type \
             {}",
            self.mode, self.instr_type
        )
    }
}

impl Error for AddrModeError {}

#[derive(Debug, Clone)]
pub struct OperandReadError {
    pub operand: Operand,
}

impl fmt::Display for OperandReadError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "invalid addressing mode {} for source operand",
            self.operand.addrmode()
        )
    }
}

impl Error for OperandReadError {}

#[derive(Debug, Clone)]
pub struct OperandAddrError {
    pub operand: Operand,
}

impl fmt::Display for OperandAddrError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "invalid addressing mode {} for destination operand",
            self.operand.addrmode()
        )
    }
}

impl Error for OperandAddrError {}

#[derive(Debug, Clone)]
pub enum MachineError {
    Read(ReadError),
    Write(WriteError),
    Bank(BankError),
    Opcode(OpcodeError),
    AddrMode(AddrModeError),
    OperandRead(OperandReadError),
    OperandAddr(OperandAddrError),
}

impl fmt::Display for MachineError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MachineError::Read(error) => write!(fmtr, "{}", error),
            MachineError::Write(error) => write!(fmtr, "{}", error),
            MachineError::Bank(error) => write!(fmtr, "{}", error),
            MachineError::Opcode(error) => write!(fmtr, "{}", error),
            MachineError::AddrMode(error) => write!(fmtr, "{}", error),
            MachineError::OperandRead(error) => write!(fmtr, "{}", error),
            MachineError::OperandAddr(error) => write!(fmtr, "{}", error),
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

impl From<AddrModeError> for MachineError {
    fn from(error: AddrModeError) -> Self {
        MachineError::AddrMode(error)
    }
}

impl From<OperandReadError> for MachineError {
    fn from(error: OperandReadError) -> Self {
        MachineError::OperandRead(error)
    }
}

impl From<OperandAddrError> for MachineError {
    fn from(error: OperandAddrError) -> Self {
        MachineError::OperandAddr(error)
    }
}

impl From<MachineError> for io::Error {
    fn from(error: MachineError) -> Self {
        let kind = match error {
            MachineError::Read(_) | MachineError::Write(_) => {
                io::ErrorKind::AddrNotAvailable
            },
            MachineError::Bank(_) => io::ErrorKind::NotFound,
            MachineError::Opcode(_)
            | MachineError::OperandRead(_)
            | MachineError::OperandAddr(_) => io::ErrorKind::InvalidData,
            MachineError::AddrMode(_) => io::ErrorKind::InvalidInput,
        };

        io::Error::new(kind, error)
    }
}
