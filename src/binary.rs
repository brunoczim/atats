pub mod decode;
pub mod encode;

pub use decode::{Decode, Decoder, NoConfig};
pub use encode::{Encode, Encoder};

macro_rules! impl_for_int {
    { $ty:ty } => {
        impl Decode for $ty {
            type Config = NoConfig;

            fn decode<D>(
                _config: &Self::Config,
                decoder: &mut D,
            ) -> Result<Self, D::Error>
            where
                D: Decoder + ?Sized,
            {
                let mut buf = [0; std::mem::size_of::<$ty>()];
                decoder.read(&mut buf)?;
                Ok(Self::from_le_bytes(buf))
            }
        }

        impl Encode for $ty {
            fn encode<E>(
                &self,
                encoder: &mut E,
            ) -> Result<(), E::Error>
            where
                E: Encoder + ?Sized
            {
                let buf = self.to_le_bytes();
                encoder.write(&buf)
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
