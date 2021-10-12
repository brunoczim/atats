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

macro_rules! impl_for_int {
    { $ty:ty } => {
        impl Decode for $ty {
            type Config = NoConfig;

            fn decode<R>(
                _config: &Self::Config,
                decoder: &mut Decoder<R>,
            ) -> io::Result<Self>
            where
                R: Read
            {
                let mut buf = [0; std::mem::size_of::<$ty>()];
                decoder.read_exact(&mut buf)?;
                Ok(Self::from_le_bytes(buf))
            }
        }
    };
}

impl_for_int! { u8 }
impl_for_int! { i8 }
impl_for_int! { u16 }
impl_for_int! { i16 }
impl_for_int! { u32 }
impl_for_int! { i32 }
impl_for_int! { u64 }
impl_for_int! { i64 }
impl_for_int! { u128 }
impl_for_int! { i128 }
