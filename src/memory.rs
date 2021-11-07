use crate::error::{BankError, ReadError, WriteError};
use std::{iter, sync::Arc};

#[derive(Debug, Clone)]
pub struct Ram {
    bytes: Box<[u8; Self::SIZE]>,
}

impl Ram {
    pub const SIZE: usize = 0x80;
    pub const OFFSET: u16 = 0x80;

    pub fn new() -> Self {
        Self { bytes: Box::new([0; Self::SIZE]) }
    }

    pub fn read(&self, address: u16) -> Result<u8, ReadError> {
        address
            .checked_sub(Self::OFFSET)
            .and_then(|actual_address| {
                self.bytes.get(usize::from(actual_address)).copied()
            })
            .ok_or(ReadError { address })
    }

    pub fn write(&mut self, address: u16, data: u8) -> Result<(), WriteError> {
        let entry = address
            .checked_sub(Self::OFFSET)
            .and_then(|actual_address| {
                self.bytes.get_mut(usize::from(actual_address))
            })
            .ok_or(WriteError { address })?;
        *entry = data;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Stack {
    bytes: Box<[u8; Self::SIZE]>,
}

impl Stack {
    pub const SIZE: usize = 0x100;

    pub const OFFSET: u16 = 0x100;

    pub fn new() -> Self {
        Self { bytes: Box::new([0; Self::SIZE]) }
    }

    pub fn read(&self, address: u16) -> Result<u8, ReadError> {
        address
            .checked_sub(Self::OFFSET)
            .and_then(|actual_address| {
                self.bytes.get(usize::from(actual_address)).copied()
            })
            .ok_or(ReadError { address })
    }

    pub fn write(&mut self, address: u16, data: u8) -> Result<(), WriteError> {
        let entry = address
            .checked_sub(Self::OFFSET)
            .and_then(|actual_address| {
                self.bytes.get_mut(usize::from(actual_address))
            })
            .ok_or(WriteError { address })?;
        *entry = data;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RomBank {
    bytes: [u8; Self::SIZE],
}

impl RomBank {
    pub const SIZE: usize = 0x1000;

    pub const OFFSET: u16 = 0xF000;

    pub fn new(content: [u8; Self::SIZE]) -> Self {
        Self { bytes: content }
    }

    pub fn try_new(slice: &[u8]) -> Option<Self> {
        let mut bytes = [0; Self::SIZE];
        if slice.len() == bytes.len() {
            bytes.copy_from_slice(slice);
            Some(Self::new(bytes))
        } else {
            None
        }
    }

    pub fn read(&self, address: u16) -> Result<u8, ReadError> {
        address
            .checked_sub(Self::OFFSET)
            .and_then(|actual_address| {
                self.bytes.get(usize::from(actual_address)).copied()
            })
            .ok_or(ReadError { address })
    }
}

#[derive(Debug, Clone)]
pub struct Rom {
    banks: Arc<[RomBank]>,
    selected: u8,
}

impl Rom {
    pub fn new<I>(default_bank: RomBank, additional_banks: I) -> Self
    where
        I: IntoIterator<Item = RomBank>,
    {
        let banks = iter::once(default_bank)
            .chain(additional_banks)
            .collect::<Vec<_>>();

        Self { banks: banks.into(), selected: 0 }
    }

    pub fn banks(&self) -> usize {
        self.banks.len()
    }

    pub fn selected_index(&self) -> u8 {
        self.selected
    }

    pub fn select_bank(&mut self, bank: u8) -> Result<(), BankError> {
        if usize::from(bank) < self.banks.len() {
            self.selected = bank;
            Ok(())
        } else {
            Err(BankError { bank })
        }
    }

    pub fn selected_bank(&self) -> &RomBank {
        &self.banks[usize::from(self.selected)]
    }

    pub fn read(&self, address: u16) -> Result<u8, ReadError> {
        self.selected_bank().read(address)
    }
}

#[derive(Debug, Clone)]
pub struct Memory {
    ram: Ram,
    stack: Stack,
    rom: Rom,
}

impl Memory {
    pub const NMI_VECTOR: u16 = 0xfffa;
    pub const RES_VECTOR: u16 = 0xfffc;
    pub const IRQ_VECTOR: u16 = 0xfffe;

    pub fn new(ram: Ram, stack: Stack, rom: Rom) -> Self {
        Self { ram, stack, rom }
    }

    pub fn rom(&self) -> Rom {
        self.rom.clone()
    }

    pub fn banks(&self) -> usize {
        self.rom.banks()
    }

    pub fn selected_bank(&self) -> u8 {
        self.rom.selected_index()
    }

    pub fn select_bank(&mut self, bank: u8) -> Result<(), BankError> {
        self.rom.select_bank(bank)
    }

    pub fn read(&self, address: u16) -> Result<u8, ReadError> {
        self.ram
            .read(address)
            .or_else(|_| self.stack.read(address))
            .or_else(|_| self.rom.read(address))
    }

    pub fn write(&mut self, address: u16, data: u8) -> Result<(), WriteError> {
        self.ram
            .write(address, data)
            .or_else(|_| self.stack.write(address, data))
    }
}
