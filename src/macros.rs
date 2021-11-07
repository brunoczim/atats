macro_rules! decode_for_wrapper {
    { $outer:ty { $field:ident: $field_ty:ty } } => {
        impl $crate::binary::Decode for $outer {
            type Config = <$field_ty as Decode>::Config;

            fn decode<D>(
                config: &Self::Config,
                decoder: &mut D,
            ) -> Result<Self, D::Error>
            where
                D: $crate::binary::Decoder + ?Sized,
            {
                let $field = decoder.decode_with(config)?;
                Ok(Self { $field })
            }
        }

        impl $crate::binary::Encode for $outer {
            fn encode<E>(
                &self,
                encoder: &mut E,
            ) -> Result<(), E::Error>
            where
                E: $crate::binary::Encoder + ?Sized
            {
                encoder.encode(&self.$field)
            }
        }
    };
}

macro_rules! decode_for_unit {
    { $outer:ty } => {
        impl $crate::binary::Decode for $outer {
            type Config = $crate::binary::NoConfig;

            fn decode<D>(
                _config: &Self::Config,
                _decoder: &mut D,
            ) -> Result<Self, D::Error>
            where
                D: $crate::binary::Decoder + ?Sized,
            {
                Ok(Self)
            }
        }

        impl $crate::binary::Encode for $outer {
            fn encode<E>(
                &self,
                _encoder: &mut E,
            ) -> Result<(), E::Error>
            where
                E: $crate::binary::Encoder + ?Sized
            {
                Ok(())
            }
        }
    };
}

macro_rules! read_using_addr {
    ($ty:ty) => {
        impl $crate::addrmode::ReadOperand for $ty {
            fn read(
                &self,
                machine: &$crate::machine::Machine,
            ) -> Result<u8, $crate::error::MachineError> {
                let address = self.address(machine)?;
                let byte = machine.memory.read(address)?;
                Ok(byte)
            }
        }
    };
}

macro_rules! display_using_render {
    { $ty:ty } => {
        impl std::fmt::Display for $ty {
            fn fmt(&self, fmtr: &mut fmt::Formatter) -> std::fmt::Result {
                use $crate::assembly::disassemble;
                let config = disassemble::Config::for_display();
                write!(
                    fmtr,
                    "{}",
                    disassemble::Renderer::with_config(self, config)
                )
            }
        }
    };
}
