use crate::arch::Address;

#[derive(Debug, Clone)]
pub enum RuntimeErrorKind {
    BadRead(Address),
    BadWrite(Address),
}
