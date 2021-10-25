use crate::{
    addrmode::Operand,
    error::{MachineError, OperandAddrError, OperandReadError},
    memory::{Memory, Ram, Rom, RomBank, Stack},
};

#[derive(Debug, Clone)]
pub struct Machine {
    memory: Memory,
    ra: u8,
    rx: u8,
    ry: u8,
    sp: u8,
    sr: Status,
    pc: u16,
}

impl Machine {
    pub fn new<I>(default_bank: RomBank, additional_banks: I) -> Self
    where
        I: IntoIterator<Item = RomBank>,
    {
        let rom = Rom::new(default_bank, additional_banks);
        let ram = Ram::new();
        let stack = Stack::new();
        let memory = Memory::new(ram, stack, rom);
        Self::from_state(memory, 0, 0, 0, 0, Status::zeroed(), 0)
    }

    pub fn from_state(
        memory: Memory,
        ra: u8,
        rx: u8,
        ry: u8,
        sp: u8,
        sr: Status,
        pc: u16,
    ) -> Self {
        Self { memory, ra, rx, ry, sp, sr, pc }
    }

    pub fn read_operand(
        &mut self,
        operand: Operand,
    ) -> Result<u8, MachineError> {
        match operand {
            Operand::Acc(_acc) => Ok(self.ra),
            Operand::Imm(imm) => Ok(imm.bits),
            Operand::Impl(_) => {
                Err(MachineError::OperandRead(OperandReadError { operand }))
            },
            _ => {
                let address = self.operand_addr(operand)?;
                let data = self.memory.read(address)?;
                Ok(data)
            },
        }
    }

    pub fn operand_addr(
        &mut self,
        operand: Operand,
    ) -> Result<u16, MachineError> {
        match operand {
            Operand::Abs(abs) => Ok(abs.address),
            Operand::AbsX(abs) => {
                Ok(abs.address.wrapping_add(u16::from(self.rx)))
            },
            Operand::AbsY(abs) => {
                Ok(abs.address.wrapping_add(u16::from(self.ry)))
            },
            Operand::Ind(ind) => {
                let low = self.memory.read(ind.address)?;
                let high = self.memory.read(ind.address.wrapping_add(1))?;
                Ok(u16::from_le_bytes([low, high]))
            },
            Operand::XInd(ind) => {
                let address = u16::from(ind.address.wrapping_add(self.rx));
                let low = self.memory.read(address)?;
                let high = self.memory.read(address.wrapping_add(1))?;
                Ok(u16::from_le_bytes([low, high]))
            },
            Operand::IndY(ind) => {
                let address = u16::from(ind.address);
                let low = self.memory.read(address)?;
                let high = self.memory.read(address.wrapping_add(1))?;
                let actual_addr = u16::from_le_bytes([low, high])
                    .wrapping_add(u16::from(self.ry));
                Ok(actual_addr)
            },
            Operand::Zpg(zpg) => Ok(u16::from(zpg.address)),
            Operand::ZpgX(zpg) => {
                Ok(u16::from(zpg.address).wrapping_add(u16::from(self.rx)))
            },
            Operand::ZpgY(zpg) => {
                Ok(u16::from(zpg.address).wrapping_add(u16::from(self.ry)))
            },
            Operand::Rel(rel) => {
                Ok(self.pc.wrapping_sub(i16::from(rel.address) as u16))
            },
            _ => Err(MachineError::OperandAddr(OperandAddrError { operand })),
        }
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
