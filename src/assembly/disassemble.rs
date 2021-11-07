use crate::{
    assembly::Syntax,
    instruction::{self, Instruction},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Disassembler {
    address: u16,
    config: Config,
}

impl Disassembler {
    pub fn new(address: u16) -> Self {
        Self::with_config(address, Config::default())
    }

    pub fn with_config(address: u16, config: Config) -> Self {
        Self { address, config }
    }

    pub fn next(&mut self, instruction: Instruction) -> instruction::Addressed {
        let addressed =
            instruction::Addressed { address: self.address, instruction };

        self.address = self.address.wrapping_add(1);
        addressed
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeywordCase {
    Lower,
    Upper,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Base {
    Decimal,
    Hex,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub syntax: Syntax,
    pub keyword_case: KeywordCase,
    pub base: Base,
    pub dump_orgs: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            syntax: Syntax::Classic,
            keyword_case: KeywordCase::Upper,
            base: Base::Decimal,
            dump_orgs: false,
        }
    }
}

impl Config {
    pub fn for_display() -> Self {
        Self { syntax: Syntax::Detailed, base: Base::Hex, ..Self::default() }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Context {
    config: Config,
}

impl Context {
    fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(self) -> Config {
        self.config
    }

    pub fn renderer<T>(self, target: T) -> Renderer<T>
    where
        T: Render,
    {
        Renderer { target, context: self }
    }
}

pub trait Render {
    fn render(
        &self,
        ctx: Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result;
}

impl<'this, T> Render for &'this T
where
    T: Render + ?Sized,
{
    fn render(
        &self,
        ctx: Context,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        (**self).render(ctx, formatter)
    }
}

macro_rules! render_for_int {
    ($ty:ty) => {
        impl Render for $ty {
            fn render(
                &self,
                ctx: Context,
                formatter: &mut fmt::Formatter,
            ) -> fmt::Result {
                match (ctx.config().base, ctx.config().keyword_case) {
                    (Base::Hex, KeywordCase::Lower) => {
                        write!(formatter, "%{:x}", self)
                    },
                    (Base::Hex, KeywordCase::Upper) => {
                        write!(formatter, "%{:X}", self)
                    },
                    (Base::Decimal, _) => write!(formatter, "{}", self),
                }
            }
        }
    };
}

render_for_int! { u8 }
render_for_int! { i8 }
render_for_int! { u16 }
render_for_int! { i16 }
render_for_int! { u32 }
render_for_int! { i32 }
render_for_int! { u64 }
render_for_int! { i64 }
render_for_int! { u128 }
render_for_int! { i128 }

#[derive(Debug, Clone, Copy)]
pub struct Renderer<T>
where
    T: Render,
{
    target: T,
    context: Context,
}

impl<T> Renderer<T>
where
    T: Render,
{
    pub fn new(target: T) -> Self {
        Self::with_config(target, Config::default())
    }

    pub fn with_config(target: T, config: Config) -> Self {
        Self { target, context: Context::new(config) }
    }
}

impl<T> fmt::Display for Renderer<T>
where
    T: Render,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.target.render(self.context, formatter)
    }
}
