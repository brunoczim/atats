use crate::{
    addrmode::{OperandAddr, ReadOperand},
    binary::{decode::MemoryDecoder, Decoder},
    error::MachineError,
    instruction::Instruction,
    memory::Memory,
};

#[derive(Debug, Clone)]
pub struct Machine {
    pub memory: Memory,
    pub ra: u8,
    pub rx: u8,
    pub ry: u8,
    pub sp: u8,
    pub sr: Status,
    pub pc: u16,
    pub fault: Option<MachineError>,
    private: (),
}

impl Machine {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            ra: 0,
            rx: 0,
            ry: 0,
            sp: 0,
            sr: Status::zeroed(),
            pc: 0,
            fault: None,
            private: (),
        }
    }

    pub fn decoder(&mut self) -> MemoryDecoder {
        MemoryDecoder::new(&self.memory, &mut self.pc)
    }

    pub fn fetch_decode(&mut self) -> Result<Instruction, MachineError> {
        self.decoder().decode()
    }

    pub fn read_operand<O>(&self, operand: O) -> Result<u8, MachineError>
    where
        O: ReadOperand,
    {
        operand.read(self)
    }

    pub fn operand_addr<O>(&self, operand: O) -> Result<u16, MachineError>
    where
        O: OperandAddr,
    {
        operand.address(self)
    }

    pub fn sp_address(&self) -> u16 {
        u16::from_le_bytes([self.sp, 1])
    }

    pub fn push_byte(&mut self, byte: u8) -> Result<(), MachineError> {
        let address = self.sp_address();
        self.memory.write(address, byte)?;
        self.sp = self.sp.wrapping_sub(1);
        Ok(())
    }

    pub fn pop_byte(&mut self) -> Result<u8, MachineError> {
        let address = self.sp_address();
        let byte = self.memory.read(address)?;
        self.sp = self.sp.wrapping_add(1);
        Ok(byte)
    }

    pub fn push_address(&mut self, address: u16) -> Result<(), MachineError> {
        for byte in address.to_le_bytes().iter().rev() {
            self.push_byte(*byte)?;
        }
        Ok(())
    }

    pub fn pop_address(&mut self) -> Result<u16, MachineError> {
        let mut bytes = [0; 2];
        for byte in &mut bytes {
            *byte = self.pop_byte()?;
        }
        Ok(u16::from_le_bytes(bytes))
    }

    pub fn push_sr(&mut self) -> Result<(), MachineError> {
        let sr = self.sr.bits();
        self.push_byte(sr)?;
        self.sr.set_unused(true);
        self.sr.set_b(true);
        Ok(())
    }

    pub fn pop_sr(&mut self) -> Result<(), MachineError> {
        let bits = self.pop_byte()?;
        let mut sr = Status::from_bits(bits);
        sr.set_b(self.sr.get_b());
        sr.set_unused(self.sr.get_unused());
        self.sr = sr;
        Ok(())
    }

    pub fn interrupt(
        &mut self,
        kind: InterruptKind,
    ) -> Result<(), MachineError> {
        if !kind.is_maskable() || !self.sr.get_i() {
            let ret_address = self.pc.wrapping_add(2);
            self.push_address(ret_address)?;
            self.push_sr()?;
            self.sr.set_i(true);

            let address = match kind {
                InterruptKind::Reset => {
                    self.sr.set_b(false);
                    Memory::RES_VECTOR
                },
                InterruptKind::NonMaskable => {
                    self.sr.set_b(false);
                    Memory::NMI_VECTOR
                },
                InterruptKind::Request => Memory::IRQ_VECTOR,
            };

            let low = self.memory.read(address)?;
            let high = self.memory.read(address.wrapping_add(1))?;
            self.pc = u16::from_le_bytes([low, high]);
        }

        Ok(())
    }

    pub fn try_step(&mut self) -> Result<(), MachineError> {
        self.fetch_decode()?.execute(self)
    }

    pub fn step(&mut self) -> Result<(), MachineError> {
        match self.try_step() {
            Ok(()) => Ok(()),
            Err(error) if self.fault.is_some() => Err(error),
            Err(error) => {
                self.fault = Some(error);
                self.interrupt(InterruptKind::NonMaskable)?;
                Ok(())
            },
        }
    }

    pub fn steps(&mut self, max_steps: u64) -> Result<(), MachineError> {
        for _ in 0 .. max_steps {
            self.step()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InterruptKind {
    NonMaskable,
    Request,
    Reset,
}

impl InterruptKind {
    pub fn is_maskable(self) -> bool {
        matches!(self, InterruptKind::Request | InterruptKind::Reset)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Status {
    flags: u8,
}

impl Status {
    const NEGATIVE: u8 = 7;
    const OVERFLOW: u8 = 6;
    const IGNORED: u8 = 5;
    const BREAK: u8 = 4;
    const DECIMAL: u8 = 3;
    const INTERRUPT: u8 = 2;
    const ZERO: u8 = 1;
    const CARRY: u8 = 0;

    pub fn zeroed() -> Self {
        Self::default()
    }

    pub fn from_bits(bits: u8) -> Self {
        Self { flags: bits }
    }

    pub fn bits(self) -> u8 {
        self.flags
    }

    fn get(&self, flag: u8) -> bool {
        self.flags & (1 << flag) != 0
    }

    fn set(&mut self, flag: u8, value: bool) {
        self.flags &= !(1 << flag);
        self.flags |= u8::from(value) << flag;
    }

    pub fn get_n(&self) -> bool {
        self.get(Self::NEGATIVE)
    }

    pub fn set_n(&mut self, value: bool) {
        self.set(Self::NEGATIVE, value)
    }

    pub fn get_v(&self) -> bool {
        self.get(Self::OVERFLOW)
    }

    pub fn set_v(&mut self, value: bool) {
        self.set(Self::OVERFLOW, value)
    }

    pub fn get_unused(&self) -> bool {
        self.get(Self::IGNORED)
    }

    pub fn set_unused(&mut self, value: bool) {
        self.set(Self::IGNORED, value)
    }

    pub fn get_b(&self) -> bool {
        self.get(Self::BREAK)
    }

    pub fn set_b(&mut self, value: bool) {
        self.set(Self::BREAK, value)
    }

    pub fn get_d(&self) -> bool {
        self.get(Self::DECIMAL)
    }

    pub fn set_d(&mut self, value: bool) {
        self.set(Self::DECIMAL, value)
    }

    pub fn get_i(&self) -> bool {
        self.get(Self::INTERRUPT)
    }

    pub fn set_i(&mut self, value: bool) {
        self.set(Self::INTERRUPT, value)
    }

    pub fn get_z(&self) -> bool {
        self.get(Self::ZERO)
    }

    pub fn set_z(&mut self, value: bool) {
        self.set(Self::ZERO, value)
    }

    pub fn get_c(&self) -> bool {
        self.get(Self::CARRY)
    }

    pub fn set_c(&mut self, value: bool) {
        self.set(Self::CARRY, value)
    }

    pub fn update_from_byte(&mut self, byte: u8) {
        self.set_z(byte == 0);
        self.set_n((byte & 0x80) != 0);
    }
}
