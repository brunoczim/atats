use crate::{
    binary::{Decode, Decoder, Encode, Encoder, NoConfig},
    error::{AddrModeError, MachineError, OperandAddrError, OperandReadError},
    instruction,
    machine::Machine,
};
use std::fmt;

macro_rules! decode_for_wrapper {
    { $outer:ty { $field:ident: $field_ty:ty } } => {
        impl Decode for $outer {
            type Config = <$field_ty as Decode>::Config;

            fn decode<D>(
                config: &Self::Config,
                decoder: &mut D,
            ) -> Result<Self, D::Error>
            where
                D: Decoder + ?Sized,
            {
                let $field = decoder.decode_with(config)?;
                Ok(Self { $field })
            }
        }

        impl Encode for $outer {
            fn encode<E>(
                &self,
                encoder: &mut E,
            ) -> Result<(), E::Error>
            where
                E: Encoder + ?Sized
            {
                encoder.encode(&self.$field)
            }
        }
    };
}

macro_rules! decode_for_unit {
    { $outer:ty } => {
        impl Decode for $outer {
            type Config = NoConfig;

            fn decode<D>(
                _config: &Self::Config,
                _decoder: &mut D,
            ) -> Result<Self, D::Error>
            where
                D: Decoder + ?Sized,
            {
                Ok(Self)
            }
        }

        impl Encode for $outer {
            fn encode<E>(
                &self,
                _encoder: &mut E,
            ) -> Result<(), E::Error>
            where
                E: Encoder + ?Sized
            {
                Ok(())
            }
        }
    };
}

macro_rules! impl_read_using_addr {
    ($ty:ty) => {
        impl ReadOperand for $ty {
            fn read(&self, machine: &Machine) -> Result<u8, MachineError> {
                let address = self.address(machine)?;
                let byte = machine.memory.read(address)?;
                Ok(byte)
            }
        }
    };
}

pub trait ReadOperand {
    fn read(&self, machine: &Machine) -> Result<u8, MachineError>;
}

impl<'this, T> ReadOperand for &'this T
where
    T: ReadOperand + ?Sized,
{
    fn read(&self, machine: &Machine) -> Result<u8, MachineError> {
        (**self).read(machine)
    }
}

pub trait OperandAddr {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError>;
}

impl<'this, T> OperandAddr for &'this T
where
    T: OperandAddr + ?Sized,
{
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        (**self).address(machine)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Absolute {
    pub address: u16,
}

impl OperandAddr for Absolute {
    fn address(&self, _machine: &Machine) -> Result<u16, MachineError> {
        Ok(self.address)
    }
}

impl fmt::Display for Absolute {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", self.address)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AbsoluteX {
    pub address: u16,
}

impl OperandAddr for AbsoluteX {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        Ok(self.address.wrapping_add(u16::from(machine.rx)))
    }
}

impl fmt::Display for AbsoluteX {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}, X", self.address)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AbsoluteY {
    pub address: u16,
}

impl OperandAddr for AbsoluteY {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        Ok(self.address.wrapping_add(u16::from(machine.ry)))
    }
}

impl fmt::Display for AbsoluteY {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}, Y", self.address)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Immediate {
    pub bits: u8,
}

impl ReadOperand for Immediate {
    fn read(&self, _machine: &Machine) -> Result<u8, MachineError> {
        Ok(self.bits)
    }
}

impl fmt::Display for Immediate {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "#{}", self.bits)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Indirect {
    pub address: u16,
}

impl OperandAddr for Indirect {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        let low = machine.memory.read(self.address)?;
        let high = machine.memory.read(self.address.wrapping_add(1))?;
        Ok(u16::from_le_bytes([low, high]))
    }
}

impl fmt::Display for Indirect {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "({})", self.address)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct XIndirect {
    pub address: u8,
}

impl OperandAddr for XIndirect {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        let address = self.address.wrapping_add(machine.rx);
        let low = machine.memory.read(u16::from(address))?;
        let high = machine.memory.read(u16::from(address.wrapping_add(1)))?;
        Ok(u16::from_le_bytes([low, high]))
    }
}

impl fmt::Display for XIndirect {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "({}, X)", self.address)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndirectY {
    pub address: u8,
}

impl OperandAddr for IndirectY {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        let low_address = u16::from(self.address);
        let low = machine.memory.read(low_address)?;
        let high_address = u16::from(self.address.wrapping_add(1));
        let high = machine.memory.read(high_address)?;
        let effective_addr = u16::from_le_bytes([low, high]);
        Ok(effective_addr.wrapping_add(u16::from(machine.ry)))
    }
}

impl fmt::Display for IndirectY {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "({}), Y", self.address)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Relative {
    pub address: i8,
}

impl OperandAddr for Relative {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        let offset = i16::from(self.address) as u16;
        Ok(machine.pc.wrapping_add(offset))
    }
}

impl fmt::Display for Relative {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "rel {}", self.address)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Zeropage {
    pub address: u8,
}

impl OperandAddr for Zeropage {
    fn address(&self, _machine: &Machine) -> Result<u16, MachineError> {
        Ok(u16::from(self.address))
    }
}

impl fmt::Display for Relative {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "0 {}", self.address)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZeropageX {
    pub address: u8,
}

impl OperandAddr for ZeropageX {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        Ok(u16::from(self.address.wrapping_add(machine.rx)))
    }
}

impl fmt::Display for ZeropageX {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "0 {}, X", self.address)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZeropageY {
    pub address: u8,
}

impl OperandAddr for ZeropageY {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        Ok(u16::from(self.address.wrapping_add(machine.ry)))
    }
}

impl fmt::Display for ZeropageY {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "0 {}, Y", self.address)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Accumulator;

impl ReadOperand for Accumulator {
    fn read(&self, machine: &Machine) -> Result<u8, MachineError> {
        Ok(machine.ra)
    }
}

impl fmt::Display for Accumulator {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "A")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Implied;

impl fmt::Display for Implied {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "")
    }
}

impl_read_using_addr! { Absolute }
impl_read_using_addr! { AbsoluteX }
impl_read_using_addr! { AbsoluteY }
impl_read_using_addr! { Indirect }
impl_read_using_addr! { XIndirect }
impl_read_using_addr! { IndirectY }
impl_read_using_addr! { Relative }
impl_read_using_addr! { Zeropage }
impl_read_using_addr! { ZeropageX }
impl_read_using_addr! { ZeropageY }

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

impl fmt::Display for AddrMode {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddrMode::Acc => write!(fmtr, "ACC"),
            AddrMode::Abs => write!(fmtr, "ABS"),
            AddrMode::AbsX => write!(fmtr, "ABSX"),
            AddrMode::AbsY => write!(fmtr, "ABSY"),
            AddrMode::Imm => write!(fmtr, "IMM"),
            AddrMode::Impl => write!(fmtr, "IMPL"),
            AddrMode::Ind => write!(fmtr, "IND"),
            AddrMode::XInd => write!(fmtr, "XIND"),
            AddrMode::IndY => write!(fmtr, "INDY"),
            AddrMode::Rel => write!(fmtr, "REL"),
            AddrMode::Zpg => write!(fmtr, "ZPG"),
            AddrMode::ZpgX => write!(fmtr, "ZPGX"),
            AddrMode::ZpgY => write!(fmtr, "ZPGY"),
        }
    }
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

    pub fn require_implied(self) -> Result<Implied, MachineError> {
        match self {
            Operand::Impl(mode) => Ok(mode),

            _ => Err(MachineError::AddrMode(AddrModeError {
                mode: self.addrmode(),
                instr_type: instruction::Type::Imp,
            })),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Acc(operand) => write!(fmtr, "{}", operand),
            Operand::Abs(operand) => write!(fmtr, "{}", operand),
            Operand::AbsX(operand) => write!(fmtr, "{}", operand),
            Operand::AbsY(operand) => write!(fmtr, "{}", operand),
            Operand::Imm(operand) => write!(fmtr, "{}", operand),
            Operand::Ind(operand) => write!(fmtr, "{}", operand),
            Operand::XInd(operand) => write!(fmtr, "{}", operand),
            Operand::IndY(operand) => write!(fmtr, "{}", operand),
            Operand::Rel(operand) => write!(fmtr, "{}", operand),
            Operand::Zpg(operand) => write!(fmtr, "{}", operand),
            Operand::ZpgX(operand) => write!(fmtr, "{}", operand),
            Operand::ZpgY(operand) => write!(fmtr, "{}", operand),
            Operand::Impl(operand) => write!(fmtr, "{}", operand),
        }
    }
}

impl ReadOperand for Operand {
    fn read(&self, machine: &Machine) -> Result<u8, MachineError> {
        match self {
            Operand::Acc(operand) => operand.read(machine),
            Operand::Abs(operand) => operand.read(machine),
            Operand::AbsX(operand) => operand.read(machine),
            Operand::AbsY(operand) => operand.read(machine),
            Operand::Imm(operand) => operand.read(machine),
            Operand::Ind(operand) => operand.read(machine),
            Operand::XInd(operand) => operand.read(machine),
            Operand::IndY(operand) => operand.read(machine),
            Operand::Rel(operand) => operand.read(machine),
            Operand::Zpg(operand) => operand.read(machine),
            Operand::ZpgX(operand) => operand.read(machine),
            Operand::ZpgY(operand) => operand.read(machine),

            Operand::Impl(_) => {
                Err(MachineError::OperandRead(OperandReadError {
                    operand: *self,
                }))
            },
        }
    }
}

impl OperandAddr for Operand {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        match self {
            Operand::Abs(operand) => operand.address(machine),
            Operand::AbsX(operand) => operand.address(machine),
            Operand::AbsY(operand) => operand.address(machine),
            Operand::Ind(operand) => operand.address(machine),
            Operand::XInd(operand) => operand.address(machine),
            Operand::IndY(operand) => operand.address(machine),
            Operand::Rel(operand) => operand.address(machine),
            Operand::Zpg(operand) => operand.address(machine),
            Operand::ZpgX(operand) => operand.address(machine),
            Operand::ZpgY(operand) => operand.address(machine),

            Operand::Acc(_) | Operand::Imm(_) | Operand::Impl(_) => {
                Err(MachineError::OperandAddr(OperandAddrError {
                    operand: *self,
                }))
            },
        }
    }
}

impl Decode for Operand {
    type Config = AddrMode;

    fn decode<D>(
        config: &Self::Config,
        decoder: &mut D,
    ) -> Result<Self, D::Error>
    where
        D: Decoder + ?Sized,
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
    fn encode<E>(&self, encoder: &mut E) -> Result<(), E::Error>
    where
        E: Encoder + ?Sized,
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
