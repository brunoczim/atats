pub mod type_aop;
pub mod type_ldx;
pub mod type_ldy;
pub mod type_sta;
pub mod type_stx;
pub mod type_sty;
pub mod type_cxy;
pub mod type_rsh;
pub mod type_idc;
pub mod type_bit;
pub mod type_jmp;
pub mod type_jsr;
pub mod type_bch;
pub mod type_imp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GenericInstr<O, A> {
    pub opcode: O,
    pub addrmode: A,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InstrKind {
    Aop(type_aop::Instruction),
    Ldx(type_ldx::Instruction),
    Ldy(type_ldy::Instruction),
    Sta(type_sta::Instruction),
    Stx(type_stx::Instruction),
    Sty(type_sty::Instruction),
    Cxy(type_cxy::Instruction),
    Rsh(type_rsh::Instruction),
    Idc(type_idc::Instruction),
    Bit(type_bit::Instruction),
    Jmp(type_jmp::Instruction),
    Jsr(type_jsr::Instruction),
    Bch(type_bch::Instruction),
    Imp(type_imp::Instruction),
}
