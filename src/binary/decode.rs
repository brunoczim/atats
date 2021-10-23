use crate::{error::MachineError, memory::Memory};
use std::io::{self, Read};

#[derive(Debug, Clone, Copy, Default)]
pub struct NoConfig;

pub trait Decode: Sized {
    type Config: ?Sized;

    fn decode<D>(
        config: &Self::Config,
        decoder: &mut D,
    ) -> Result<Self, D::Error>
    where
        D: Decoder + ?Sized;
}

pub trait Decoder {
    type Error: From<MachineError>;

    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;

    fn decode<T>(&mut self) -> Result<T, Self::Error>
    where
        T: Decode<Config = NoConfig>,
    {
        self.decode_with(&NoConfig)
    }

    fn decode_with<T>(&mut self, config: &T::Config) -> Result<T, Self::Error>
    where
        T: Decode,
    {
        T::decode(config, self)
    }
}

#[derive(Debug)]
pub struct MemoryDecoder<'mem, 'pc> {
    memory: &'mem Memory,
    pc: &'pc mut u16,
}

impl<'mem, 'pc> MemoryDecoder<'mem, 'pc> {
    pub fn new(memory: &'mem Memory, pc: &'pc mut u16) -> Self {
        Self { memory, pc }
    }

    pub fn memory(&self) -> &'mem Memory {
        self.memory
    }

    pub fn pc(&self) -> u16 {
        *self.pc
    }

    pub fn pc_mut(&mut self) -> &mut u16 {
        self.pc
    }
}

impl<'mem, 'pc> Decoder for MemoryDecoder<'mem, 'pc> {
    type Error = MachineError;

    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        for byte in buf {
            let result = self.memory.read(*self.pc);
            if result.is_ok() {
                *self.pc = self.pc.wrapping_add(1);
            }
            *byte = result?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct IoDecoder<R>
where
    R: Read,
{
    reader: R,
}

impl<R> IoDecoder<R>
where
    R: Read,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn into_reader(self) -> R {
        self.reader
    }
}

impl<R> Decoder for IoDecoder<R>
where
    R: Read,
{
    type Error = io::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.reader.read_exact(buf)
    }
}
