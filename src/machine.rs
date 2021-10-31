use crate::{
    addrmode::{OperandAddr, ReadOperand},
    error::MachineError,
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
            private: (),
        }
    }

    pub fn read_operand<O>(&mut self, operand: O) -> Result<u8, MachineError>
    where
        O: ReadOperand,
    {
        operand.read(self)
    }

    pub fn operand_addr<O>(&mut self, operand: O) -> Result<u16, MachineError>
    where
        O: OperandAddr,
    {
        operand.address(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Status {
    flags: u8,
}

impl Status {
    const NEGATIVE: u8 = 7;
    const OVERFLOW: u8 = 6;
    const BREAK: u8 = 4;
    const DECIMAL: u8 = 3;
    const INTERRUPT: u8 = 2;
    const ZERO: u8 = 1;
    const CARRY: u8 = 0;

    pub fn zeroed() -> Self {
        Self::default()
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
}
