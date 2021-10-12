use crate::arch::{Address, PageAddr, Word, WordBits};

#[derive(Debug, Clone)]
pub struct Machine {
    memory: Box<[Word]>,
    ra: Word,
    rx: Word,
    ry: Word,
    sp: PageAddr,
    sr: Status,
    pc: Address,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Status {
    flags: Word,
}

impl Status {
    const NEGATIVE: WordBits = 7;
    const OVERFLOW: WordBits = 6;
    const BREAK: WordBits = 4;
    const DECIMAL: WordBits = 3;
    const INTERRUPT: WordBits = 2;
    const ZERO: WordBits = 1;
    const CARRY: WordBits = 0;

    pub fn zeroed() -> Self {
        Self::default()
    }

    fn get(&self, flag: WordBits) -> bool {
        self.flags.bits & (1 << flag) != 0
    }

    fn set(&mut self, flag: WordBits, value: bool) {
        self.flags.bits &= !(1 << flag);
        self.flags.bits |= WordBits::from(value) << flag;
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
