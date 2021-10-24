use crate::error::MachineError;
use std::io::{self, Write};

pub trait Encode {
    fn encode<E>(&self, encoder: &mut E) -> Result<(), E::Error>
    where
        E: Encoder + ?Sized;
}

impl<'this, T> Encode for &'this T
where
    T: Encode + ?Sized,
{
    fn encode<E>(&self, encoder: &mut E) -> Result<(), E::Error>
    where
        E: Encoder + ?Sized,
    {
        (**self).encode(encoder)
    }
}

pub trait Encoder {
    type Error: From<MachineError>;

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error>;

    fn encode<T>(&mut self, data: T) -> Result<(), Self::Error>
    where
        T: Encode,
    {
        data.encode(self)
    }
}

#[derive(Debug, Clone)]
pub struct IoEncoder<W>
where
    W: Write,
{
    writer: W,
}

impl<W> IoEncoder<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn into_writer(self) -> W {
        self.writer
    }
}

impl<W> Encoder for IoEncoder<W>
where
    W: Write,
{
    type Error = io::Error;

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.writer.write_all(buf)
    }
}

#[derive(Debug)]
pub struct VecEncoder<'buf> {
    output: &'buf mut Vec<u8>,
}

impl<'buf> VecEncoder<'buf> {
    pub fn new(output: &'buf mut Vec<u8>) -> Self {
        Self { output }
    }
}

impl<'buf> Encoder for VecEncoder<'buf> {
    type Error = MachineError;

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.output.extend_from_slice(buf);
        Ok(())
    }
}
