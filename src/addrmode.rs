use crate::{
    assembly::{self, disassemble},
    binary::{Decode, Decoder, Encode, Encoder},
    error::{AddrModeError, MachineError, OperandAddrError, OperandReadError},
    instruction,
    machine::Machine,
};
use std::fmt;

pub trait OperandSize {
    fn size(&self) -> usize;

    fn renders_empty(&self, _ctx: disassemble::Context) -> bool {
        false
    }
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

impl OperandSize for Absolute {
    fn size(&self) -> usize {
        2
    }
}

impl OperandAddr for Absolute {
    fn address(&self, _machine: &Machine) -> Result<u16, MachineError> {
        Ok(self.address)
    }
}

impl disassemble::Render for Absolute {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(formatter, "{}", ctx.renderer(self.address))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AbsoluteX {
    pub address: u16,
}

impl OperandSize for AbsoluteX {
    fn size(&self) -> usize {
        2
    }
}

impl OperandAddr for AbsoluteX {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        Ok(self.address.wrapping_add(u16::from(machine.rx)))
    }
}

impl disassemble::Render for AbsoluteX {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match ctx.config().keyword_case {
            disassemble::KeywordCase::Lower => {
                write!(formatter, "{}, x", ctx.renderer(self.address))
            },
            disassemble::KeywordCase::Upper => {
                write!(formatter, "{}, X", ctx.renderer(self.address))
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AbsoluteY {
    pub address: u16,
}

impl OperandSize for AbsoluteY {
    fn size(&self) -> usize {
        2
    }
}

impl OperandAddr for AbsoluteY {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        Ok(self.address.wrapping_add(u16::from(machine.ry)))
    }
}

impl disassemble::Render for AbsoluteY {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match ctx.config().keyword_case {
            disassemble::KeywordCase::Lower => {
                write!(formatter, "{}, y", ctx.renderer(self.address))
            },
            disassemble::KeywordCase::Upper => {
                write!(formatter, "{}, Y", ctx.renderer(self.address))
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Immediate {
    pub bits: u8,
}

impl OperandSize for Immediate {
    fn size(&self) -> usize {
        1
    }
}

impl ReadOperand for Immediate {
    fn read(&self, _machine: &Machine) -> Result<u8, MachineError> {
        Ok(self.bits)
    }
}

impl disassemble::Render for Immediate {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(formatter, "#{}", ctx.renderer(self.bits))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Indirect {
    pub address: u16,
}

impl OperandSize for Indirect {
    fn size(&self) -> usize {
        2
    }
}

impl OperandAddr for Indirect {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        let low = machine.memory.read(self.address)?;
        let high = machine.memory.read(self.address.wrapping_add(1))?;
        Ok(u16::from_le_bytes([low, high]))
    }
}

impl disassemble::Render for Indirect {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(formatter, "({})", ctx.renderer(self.address))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct XIndirect {
    pub address: u8,
}

impl OperandSize for XIndirect {
    fn size(&self) -> usize {
        1
    }
}

impl OperandAddr for XIndirect {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        let address = self.address.wrapping_add(machine.rx);
        let low = machine.memory.read(u16::from(address))?;
        let high = machine.memory.read(u16::from(address.wrapping_add(1)))?;
        Ok(u16::from_le_bytes([low, high]))
    }
}

impl disassemble::Render for XIndirect {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match ctx.config().keyword_case {
            disassemble::KeywordCase::Lower => {
                write!(formatter, "({}, x)", ctx.renderer(self.address))
            },
            disassemble::KeywordCase::Upper => {
                write!(formatter, "({}, X)", ctx.renderer(self.address))
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndirectY {
    pub address: u8,
}

impl OperandSize for IndirectY {
    fn size(&self) -> usize {
        1
    }
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

impl disassemble::Render for IndirectY {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match ctx.config().keyword_case {
            disassemble::KeywordCase::Lower => {
                write!(formatter, "({}), y", ctx.renderer(self.address))
            },
            disassemble::KeywordCase::Upper => {
                write!(formatter, "({}), Y", ctx.renderer(self.address))
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Relative {
    pub address: i8,
}

impl OperandSize for Relative {
    fn size(&self) -> usize {
        1
    }
}

impl OperandAddr for Relative {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        let offset = i16::from(self.address) as u16;
        Ok(machine.pc.wrapping_add(offset))
    }
}

impl disassemble::Render for Relative {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match (ctx.config().syntax, ctx.config().keyword_case) {
            (assembly::Syntax::Detailed, disassemble::KeywordCase::Lower) => {
                write!(formatter, "rel {}", ctx.renderer(self.address))
            },
            (assembly::Syntax::Detailed, disassemble::KeywordCase::Upper) => {
                write!(formatter, "REL {}", ctx.renderer(self.address))
            },
            _ => write!(formatter, "{}", ctx.renderer(self.address)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Zeropage {
    pub address: u8,
}

impl OperandSize for Zeropage {
    fn size(&self) -> usize {
        1
    }
}

impl OperandAddr for Zeropage {
    fn address(&self, _machine: &Machine) -> Result<u16, MachineError> {
        Ok(u16::from(self.address))
    }
}

impl disassemble::Render for Zeropage {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match ctx.config().syntax {
            assembly::Syntax::Detailed => {
                write!(formatter, "0, {}", ctx.renderer(self.address))
            },
            _ => write!(formatter, "{}", ctx.renderer(self.address)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZeropageX {
    pub address: u8,
}

impl OperandSize for ZeropageX {
    fn size(&self) -> usize {
        1
    }
}

impl OperandAddr for ZeropageX {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        Ok(u16::from(self.address.wrapping_add(machine.rx)))
    }
}

impl disassemble::Render for ZeropageX {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match (ctx.config().syntax, ctx.config().keyword_case) {
            (assembly::Syntax::Detailed, disassemble::KeywordCase::Lower) => {
                write!(formatter, "0, {}, x", ctx.renderer(self.address))
            },
            (assembly::Syntax::Detailed, disassemble::KeywordCase::Upper) => {
                write!(formatter, "0, {}, X", ctx.renderer(self.address))
            },

            (_, disassemble::KeywordCase::Lower) => {
                write!(formatter, "{}, x", ctx.renderer(self.address))
            },
            (_, disassemble::KeywordCase::Upper) => {
                write!(formatter, "{}, X", ctx.renderer(self.address))
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ZeropageY {
    pub address: u8,
}

impl OperandSize for ZeropageY {
    fn size(&self) -> usize {
        1
    }
}

impl OperandAddr for ZeropageY {
    fn address(&self, machine: &Machine) -> Result<u16, MachineError> {
        Ok(u16::from(self.address.wrapping_add(machine.ry)))
    }
}

impl disassemble::Render for ZeropageY {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match (ctx.config().syntax, ctx.config().keyword_case) {
            (assembly::Syntax::Detailed, disassemble::KeywordCase::Lower) => {
                write!(formatter, "0, {}, y", ctx.renderer(self.address))
            },
            (assembly::Syntax::Detailed, disassemble::KeywordCase::Upper) => {
                write!(formatter, "0, {}, Y", ctx.renderer(self.address))
            },

            (_, disassemble::KeywordCase::Lower) => {
                write!(formatter, "{}, y", ctx.renderer(self.address))
            },
            (_, disassemble::KeywordCase::Upper) => {
                write!(formatter, "{}, Y", ctx.renderer(self.address))
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Accumulator;

impl OperandSize for Accumulator {
    fn size(&self) -> usize {
        0
    }
}

impl ReadOperand for Accumulator {
    fn read(&self, machine: &Machine) -> Result<u8, MachineError> {
        Ok(machine.ra)
    }
}

impl disassemble::Render for Accumulator {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match ctx.config().keyword_case {
            disassemble::KeywordCase::Lower => write!(formatter, "a"),
            disassemble::KeywordCase::Upper => write!(formatter, "A"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Implied;

impl OperandSize for Implied {
    fn size(&self) -> usize {
        0
    }

    fn renders_empty(&self, ctx: disassemble::Context) -> bool {
        match ctx.config().syntax {
            assembly::Syntax::Detailed => false,
            _ => true,
        }
    }
}

impl disassemble::Render for Implied {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match (ctx.config().syntax, ctx.config().keyword_case) {
            (assembly::Syntax::Detailed, disassemble::KeywordCase::Lower) => {
                write!(formatter, "IMPL")
            },
            (assembly::Syntax::Detailed, disassemble::KeywordCase::Upper) => {
                write!(formatter, "impl")
            },
            _ => Ok(()),
        }
    }
}

read_using_addr! { Absolute }
read_using_addr! { AbsoluteX }
read_using_addr! { AbsoluteY }
read_using_addr! { Indirect }
read_using_addr! { XIndirect }
read_using_addr! { IndirectY }
read_using_addr! { Relative }
read_using_addr! { Zeropage }
read_using_addr! { ZeropageX }
read_using_addr! { ZeropageY }

display_using_render! { Absolute }
display_using_render! { AbsoluteX }
display_using_render! { AbsoluteY }
display_using_render! { Immediate }
display_using_render! { Indirect }
display_using_render! { XIndirect }
display_using_render! { IndirectY }
display_using_render! { Relative }
display_using_render! { Zeropage }
display_using_render! { ZeropageX }
display_using_render! { ZeropageY }
display_using_render! { Accumulator }
display_using_render! { Implied }
display_using_render! { Operand }

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

impl OperandSize for Operand {
    fn size(&self) -> usize {
        match self {
            Operand::Acc(operand) => operand.size(),
            Operand::Abs(operand) => operand.size(),
            Operand::AbsX(operand) => operand.size(),
            Operand::AbsY(operand) => operand.size(),
            Operand::Imm(operand) => operand.size(),
            Operand::Ind(operand) => operand.size(),
            Operand::XInd(operand) => operand.size(),
            Operand::IndY(operand) => operand.size(),
            Operand::Rel(operand) => operand.size(),
            Operand::Zpg(operand) => operand.size(),
            Operand::ZpgX(operand) => operand.size(),
            Operand::ZpgY(operand) => operand.size(),
            Operand::Impl(operand) => operand.size(),
        }
    }

    fn renders_empty(&self, ctx: disassemble::Context) -> bool {
        match self {
            Operand::Acc(operand) => operand.renders_empty(ctx),
            Operand::Abs(operand) => operand.renders_empty(ctx),
            Operand::AbsX(operand) => operand.renders_empty(ctx),
            Operand::AbsY(operand) => operand.renders_empty(ctx),
            Operand::Imm(operand) => operand.renders_empty(ctx),
            Operand::Ind(operand) => operand.renders_empty(ctx),
            Operand::XInd(operand) => operand.renders_empty(ctx),
            Operand::IndY(operand) => operand.renders_empty(ctx),
            Operand::Rel(operand) => operand.renders_empty(ctx),
            Operand::Zpg(operand) => operand.renders_empty(ctx),
            Operand::ZpgX(operand) => operand.renders_empty(ctx),
            Operand::ZpgY(operand) => operand.renders_empty(ctx),
            Operand::Impl(operand) => operand.renders_empty(ctx),
        }
    }
}

impl disassemble::Render for Operand {
    fn render(
        &self,
        ctx: disassemble::Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            Operand::Acc(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
            },
            Operand::Abs(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
            },
            Operand::AbsX(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
            },
            Operand::AbsY(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
            },
            Operand::Imm(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
            },
            Operand::Ind(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
            },
            Operand::XInd(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
            },
            Operand::IndY(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
            },
            Operand::Rel(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
            },
            Operand::Zpg(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
            },
            Operand::ZpgX(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
            },
            Operand::ZpgY(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
            },
            Operand::Impl(operand) => {
                disassemble::Render::render(operand, ctx, formatter)
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
