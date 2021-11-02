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
    machine::Machine,
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

            Mnemonic::Bpl => {
                ();
            },

            Mnemonic::Bmi => {
                ();
            },

            Mnemonic::Bvc => {
                ();
            },

            Mnemonic::Bvs => {
                ();
            },

            Mnemonic::Bcc => {
                ();
            },

            Mnemonic::Bcs => {
                ();
            },

            Mnemonic::Bne => {
                ();
            },

            Mnemonic::Beq => {
                ();
            },

            Mnemonic::Bit => {
                ();
            },

            Mnemonic::Cpx => {
                ();
            },

            Mnemonic::Cpy => {
                ();
            },

            Mnemonic::Inc => {
                ();
            },

            Mnemonic::Dec => {
                ();
            },

            Mnemonic::Inx => {
                ();
            },

            Mnemonic::Iny => {
                ();
            },

            Mnemonic::Dex => {
                ();
            },

            Mnemonic::Dey => {
                ();
            },

            Mnemonic::Brk => {
                ();
            },

            Mnemonic::Php => {
                ();
            },

            Mnemonic::Rti => {
                ();
            },

            Mnemonic::Rts => {
                ();
            },

            Mnemonic::Clc => {
                ();
            },

            Mnemonic::Plp => {
                ();
            },

            Mnemonic::Sec => {
                ();
            },

            Mnemonic::Pha => {
                ();
            },

            Mnemonic::Cli => {
                ();
            },

            Mnemonic::Pla => {
                ();
            },

            Mnemonic::Sei => {
                ();
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

            Mnemonic::Clv => {
                ();
            },

            Mnemonic::Cld => {
                ();
            },

            Mnemonic::Sed => {
                ();
            },

            Mnemonic::Nop => {
                ();
            },

            Mnemonic::Jmp => {
                ();
            },

            Mnemonic::Jsr => {
                ();
            },

            Mnemonic::Ldx => {
                machine.rx = machine.read_operand(self.operand)?;
                machine.sr.update_from_byte(machine.rx);
            },

            Mnemonic::Ldy => {
                machine.ry = machine.read_operand(self.operand)?;
                machine.sr.update_from_byte(machine.ry);
            },

            Mnemonic::Asl => {
                ();
            },

            Mnemonic::Rol => {
                ();
            },

            Mnemonic::Lsr => {
                ();
            },

            Mnemonic::Ror => {
                ();
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
