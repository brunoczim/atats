use std::io::{self, Read};

#[derive(Debug, Clone)]
pub struct Decoder<R>
where
    R: Read,
{
    reader: R,
    total_read: usize,
}

impl<R> Decoder<R>
where
    R: Read,
{
    pub fn new(reader: R) -> Self {
        Self { reader, total_read: 0 }
    }

    pub fn total_read(&self) -> usize {
        self.total_read
    }

    pub fn clear(&mut self) {
        self.total_read = 0;
    }

    pub fn decode<T>(&mut self) -> io::Result<T>
    where
        T: Decode<Config = NoConfig>,
    {
        self.decode_with(&NoConfig)
    }

    pub fn decode_with<T>(&mut self, config: &T::Config) -> io::Result<T>
    where
        T: Decode,
    {
        T::decode(config, self)
    }
}

impl<R> Read for Decoder<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let count = self.reader.read(buf)?;
        self.total_read = self.total_read.saturating_add(count);
        Ok(count)
    }
}

pub trait Decode: Sized {
    type Config: ?Sized;

    fn decode<R>(
        config: &Self::Config,
        decoder: &mut Decoder<R>,
    ) -> io::Result<Self>
    where
        R: Read;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NoConfig;
