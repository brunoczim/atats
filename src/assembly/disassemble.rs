use crate::{assembly::Syntax, instruction::Instruction};
use std::fmt;

macro_rules! impl_render_for_int {
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

pub struct Disassembler<I, E>
where
    I: Iterator<Item = Result<Instruction, E>>,
{
    instructions: I,
    address: u16,
}

impl<I, E> Disassembler<I, E>
where
    I: Iterator<Item = Result<Instruction, E>>,
{
    pub fn new<J>(instructions: J, address: u16) -> Self
    where
        J: IntoIterator<IntoIter = I, Item = Result<Instruction, E>>,
    {
        Self { instructions: instructions.into_iter(), address }
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

impl_render_for_int! { u8 }
impl_render_for_int! { i8 }
impl_render_for_int! { u16 }
impl_render_for_int! { i16 }
impl_render_for_int! { u32 }
impl_render_for_int! { i32 }
impl_render_for_int! { u64 }
impl_render_for_int! { i64 }
impl_render_for_int! { u128 }
impl_render_for_int! { i128 }

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
