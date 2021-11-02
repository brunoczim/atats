pub mod opcode;
mod itype;
mod mnemonic;

pub use itype::Type;
pub use mnemonic::Mnemonic;
pub use opcode::Opcode;

use crate::{
    addrmode::Operand,
    binary::{Decode, Decoder, Encode, Encoder, NoConfig},
    error::MachineError,
    machine::{Machine, Status},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operand: Operand,
}

impl Instruction {
    pub fn opcode(self) -> Opcode {
        Opcode { mnemonic: self.mnemonic, addrmode: self.operand.addrmode() }
    }

    pub fn execute(self, machine: &mut Machine) -> Result<(), MachineError> {
        match self.mnemonic {
            Mnemonic::Ora => {
                machine.ra |= machine.read_operand(self.operand)?;
                machine.sr.update_from_byte(machine.ra);
            },

            Mnemonic::And => {
                machine.ra &= machine.read_operand(self.operand)?;
                machine.sr.update_from_byte(machine.ra);
            },

            Mnemonic::Eor => {
                machine.ra ^= machine.read_operand(self.operand)?;
                machine.sr.update_from_byte(machine.ra);
            },

            Mnemonic::Adc => {
                let carry_u = u8::from(machine.sr.get_c());
                let carry_i = carry_u as i8;

                let ra_u = machine.ra;
                let ra_i = ra_u as i8;

                let operand_u = machine.read_operand(self.operand)?;
                let operand_i = operand_u as i8;

                let (partial_u, first_cout) = ra_u.overflowing_add(operand_u);
                let (partial_i, first_ovflow) = ra_i.overflowing_add(operand_i);

                let (result, second_cout) = partial_u.overflowing_add(carry_u);
                let (_, second_ovflow) = partial_i.overflowing_add(carry_i);

                machine.ra = result;
                machine.sr.update_from_byte(machine.ra);
                machine.sr.set_c(first_cout || second_cout);
                machine.sr.set_v(first_ovflow || second_ovflow);
            },

            Mnemonic::Lda => {
                machine.ra = machine.read_operand(self.operand)?;
                machine.sr.update_from_byte(machine.ra);
            },

            Mnemonic::Ldx => {
                machine.rx = machine.read_operand(self.operand)?;
                machine.sr.update_from_byte(machine.rx);
            },

            Mnemonic::Ldy => {
                machine.ry = machine.read_operand(self.operand)?;
                machine.sr.update_from_byte(machine.ry);
            },

            Mnemonic::Cmp => {
                let operand = machine.read_operand(self.operand)?;
                let (result, borrow) = machine.ra.overflowing_sub(operand);
                machine.sr.update_from_byte(result);
                machine.sr.set_c(!borrow);
            },

            Mnemonic::Sbc => {
                let borrow_u = u8::from(!machine.sr.get_c());
                let borrow_i = borrow_u as i8;

                let ra_u = machine.ra;
                let ra_i = ra_u as i8;

                let operand_u = machine.read_operand(self.operand)?;
                let operand_i = operand_u as i8;

                let (partial_u, first_bout) = ra_u.overflowing_sub(operand_u);
                let (partial_i, first_ovflow) = ra_i.overflowing_sub(operand_i);

                let (result, second_bout) = partial_u.overflowing_sub(borrow_u);
                let (_, second_ovflow) = partial_i.overflowing_sub(borrow_i);

                machine.ra = result;
                machine.sr.update_from_byte(machine.ra);
                machine.sr.set_c(!first_bout && !second_bout);
                machine.sr.set_v(first_ovflow || second_ovflow);
            },

            Mnemonic::Cpx => {
                let operand = machine.read_operand(self.operand)?;
                let (result, borrow) = machine.rx.overflowing_sub(operand);
                machine.sr.update_from_byte(result);
                machine.sr.set_c(!borrow);
            },

            Mnemonic::Cpy => {
                let operand = machine.read_operand(self.operand)?;
                let (result, borrow) = machine.ry.overflowing_sub(operand);
                machine.sr.update_from_byte(result);
                machine.sr.set_c(!borrow);
            },

            Mnemonic::Inc => {
                let address = machine.operand_addr(self.operand)?;
                let result = machine.memory.read(address)?.wrapping_add(1);
                machine.sr.update_from_byte(result);
                machine.memory.write(address, result)?;
            },

            Mnemonic::Dec => {
                let address = machine.operand_addr(self.operand)?;
                let result = machine.memory.read(address)?.wrapping_sub(1);
                machine.sr.update_from_byte(result);
                machine.memory.write(address, result)?;
            },

            Mnemonic::Inx => {
                self.operand.require_implied()?;
                machine.rx = machine.rx.wrapping_add(1);
                machine.sr.update_from_byte(machine.rx);
            },

            Mnemonic::Iny => {
                self.operand.require_implied()?;
                machine.ry = machine.ry.wrapping_add(1);
                machine.sr.update_from_byte(machine.ry);
            },

            Mnemonic::Dex => {
                self.operand.require_implied()?;
                machine.rx = machine.rx.wrapping_sub(1);
                machine.sr.update_from_byte(machine.rx);
            },

            Mnemonic::Dey => {
                self.operand.require_implied()?;
                machine.ry = machine.ry.wrapping_sub(1);
                machine.sr.update_from_byte(machine.ry);
            },

            Mnemonic::Pha => {
                self.operand.require_implied()?;
                let acc = machine.ra;
                machine.push_byte(acc)?;
            },

            Mnemonic::Pla => {
                self.operand.require_implied()?;
                machine.ra = machine.pop_byte()?;
            },

            Mnemonic::Php => {
                self.operand.require_implied()?;
                let sr = machine.sr.bits();
                machine.push_byte(sr)?;
            },

            Mnemonic::Plp => {
                self.operand.require_implied()?;
                let sr = machine.pop_byte()?;
                machine.sr = Status::from_bits(sr);
                machine.sr.set_b(false);
            },

            Mnemonic::Brk => {
                self.operand.require_implied()?;

                let ret_address = machine.pc.wrapping_add(2);
                machine.push_address(ret_address)?;

                let sr = machine.sr.bits();
                machine.push_byte(sr)?;
                machine.sr.set_b(true);

                todo!()
            },

            Mnemonic::Rti => {
                self.operand.require_implied()?;

                let sr = machine.pop_byte()?;
                machine.sr = Status::from_bits(sr);
                machine.sr.set_b(false);

                machine.pc = machine.pop_address()?;
            },

            Mnemonic::Rts => {
                self.operand.require_implied()?;
                machine.pc = machine.pop_address()?.wrapping_add(1);
            },

            Mnemonic::Jmp => {
                machine.pc = machine.operand_addr(self.operand)?;
            },

            Mnemonic::Jsr => {
                let ret_address = machine.pc.wrapping_add(2);
                machine.push_address(ret_address)?;
                machine.pc = machine.operand_addr(self.operand)?;
            },

            Mnemonic::Bpl => {
                if !machine.sr.get_n() {
                    machine.pc = machine.operand_addr(self.operand)?;
                }
            },

            Mnemonic::Bmi => {
                if machine.sr.get_n() {
                    machine.pc = machine.operand_addr(self.operand)?;
                }
            },

            Mnemonic::Bvc => {
                if !machine.sr.get_v() {
                    machine.pc = machine.operand_addr(self.operand)?;
                }
            },

            Mnemonic::Bvs => {
                if machine.sr.get_v() {
                    machine.pc = machine.operand_addr(self.operand)?;
                }
            },

            Mnemonic::Bcc => {
                if !machine.sr.get_c() {
                    machine.pc = machine.operand_addr(self.operand)?;
                }
            },

            Mnemonic::Bcs => {
                if machine.sr.get_c() {
                    machine.pc = machine.operand_addr(self.operand)?;
                }
            },

            Mnemonic::Bne => {
                if !machine.sr.get_z() {
                    machine.pc = machine.operand_addr(self.operand)?;
                }
            },

            Mnemonic::Beq => {
                if machine.sr.get_z() {
                    machine.pc = machine.operand_addr(self.operand)?;
                }
            },

            Mnemonic::Bit => {
                let operand = machine.read_operand(self.operand)?;
                let result = machine.ra & operand;
                machine.sr.set_n((operand & 0x80) != 0);
                machine.sr.set_v((operand & 0x40) != 0);
                machine.sr.set_z(result == 0);
            },

            Mnemonic::Sec => {
                self.operand.require_implied()?;
                machine.sr.set_c(true);
            },

            Mnemonic::Clc => {
                self.operand.require_implied()?;
                machine.sr.set_c(false);
            },

            Mnemonic::Cli => {
                self.operand.require_implied()?;
                machine.sr.set_i(false);
            },

            Mnemonic::Sei => {
                self.operand.require_implied()?;
                machine.sr.set_i(true);
            },

            Mnemonic::Clv => {
                self.operand.require_implied()?;
                machine.sr.set_v(false);
            },

            Mnemonic::Cld => {
                self.operand.require_implied()?;
                machine.sr.set_d(false);
            },

            Mnemonic::Sed => {
                self.operand.require_implied()?;
                machine.sr.set_d(true);
            },

            Mnemonic::Nop => {
                self.operand.require_implied()?;
            },

            Mnemonic::Tya => {
                self.operand.require_implied()?;
                machine.ra = machine.ry;
                machine.sr.update_from_byte(machine.ra);
            },

            Mnemonic::Tay => {
                self.operand.require_implied()?;
                machine.ry = machine.ra;
                machine.sr.update_from_byte(machine.ry);
            },

            Mnemonic::Txa => {
                self.operand.require_implied()?;
                machine.ra = machine.rx;
                machine.sr.update_from_byte(machine.ra);
            },

            Mnemonic::Tax => {
                self.operand.require_implied()?;
                machine.rx = machine.ra;
                machine.sr.update_from_byte(machine.rx);
            },

            Mnemonic::Txs => {
                self.operand.require_implied()?;
                machine.sp = machine.rx;
                machine.sr.update_from_byte(machine.sp);
            },

            Mnemonic::Tsx => {
                self.operand.require_implied()?;
                machine.rx = machine.sp;
                machine.sr.update_from_byte(machine.rx);
            },

            Mnemonic::Asl => {
                let address = machine.operand_addr(self.operand)?;
                let data = machine.memory.read(address)?;
                let result = data << 1;
                machine.sr.update_from_byte(result);
                machine.sr.set_c((data & 0x80) != 0);
                machine.memory.write(address, result)?;
            },

            Mnemonic::Rol => {
                let carry_in = u8::from(machine.sr.get_c());
                let address = machine.operand_addr(self.operand)?;
                let data = machine.memory.read(address)?;
                let result = (data << 1) | carry_in;
                machine.sr.update_from_byte(result);
                machine.sr.set_c((data & 0x80) != 0);
                machine.memory.write(address, result)?;
            },

            Mnemonic::Lsr => {
                let address = machine.operand_addr(self.operand)?;
                let data = machine.memory.read(address)?;
                let result = data >> 1;
                machine.sr.update_from_byte(result);
                machine.sr.set_c((data & 0x1) != 0);
                machine.memory.write(address, result)?;
            },

            Mnemonic::Ror => {
                let carry_in = u8::from(machine.sr.get_c()) << 7;
                let address = machine.operand_addr(self.operand)?;
                let data = machine.memory.read(address)?;
                let result = (data >> 1) | carry_in;
                machine.sr.update_from_byte(result);
                machine.sr.set_c((data & 0x1) != 0);
                machine.memory.write(address, result)?;
            },

            Mnemonic::Sta => {
                let address = machine.operand_addr(self.operand)?;
                machine.memory.write(address, machine.ra)?;
            },

            Mnemonic::Stx => {
                let address = machine.operand_addr(self.operand)?;
                machine.memory.write(address, machine.rx)?;
            },

            Mnemonic::Sty => {
                let address = machine.operand_addr(self.operand)?;
                machine.memory.write(address, machine.ry)?;
            },
        }

        Ok(())
    }
}

impl Encode for Instruction {
    fn encode<E>(&self, encoder: &mut E) -> Result<(), E::Error>
    where
        E: Encoder + ?Sized,
    {
        encoder.encode(self.opcode())?;
        encoder.encode(self.operand)?;
        Ok(())
    }
}

impl Decode for Instruction {
    type Config = NoConfig;

    fn decode<D>(
        _config: &Self::Config,
        decoder: &mut D,
    ) -> Result<Self, D::Error>
    where
        D: Decoder + ?Sized,
    {
        let opcode = decoder.decode::<Opcode>()?;
        let operand = decoder.decode_with(&opcode.addrmode)?;
        Ok(Self { mnemonic: opcode.mnemonic, operand })
    }
}
