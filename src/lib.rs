// #![no_std]

extern crate alloc;

mod literal;
mod repeat;
mod error;
mod meta;

#[cfg(test)]
mod tests;

use literal::Literal;
use error::{Trace, Unknown};
use meta::{IsMeta, Meta};
use crate::error::Raise;

trait Validate {
    fn validate(&self, context: &mut Context, order: Order, last: bool) -> Result<Option<Unknown>, Trace>;
}

#[derive(Debug)]
pub enum Validator {
    Literal(&'static Literal),
    Meta(&'static Meta),
}

impl Validate for Validator {
    fn validate(&self, context: &mut Context, order: Order, last: bool) -> Result<Option<Unknown>, Trace> {
        match self {
            Validator::Literal(literal) => literal.validate(context, order, last),
            Validator::Meta(meta) => meta.validate(context, order, last),
        }
    }
}

#[derive(Debug)]
pub struct Context<Root = ()> {
    root: core::marker::PhantomData<Root>,
    args: &'static [Argument],
    current: usize,
}

impl Context {
    fn next(&mut self) -> Option<&Argument> {
        if let Some(arg) = self.args.get(self.current) {
            self.current += 1;
            Some(arg)
        } else {
            None
        }
    }

    fn back(&mut self) {
        self.current -= 1;
    }
}

pub enum Order {
    Pre,
    Post,
}

pub type Argument = &'static str;

fn validate<Root: IsMeta>(args: &'static [Argument]) -> (Context<Root>, Option<Trace>) {
    let mut context = Context { root: core::marker::PhantomData, args, current: 0 };
    let result = Root::META.validate(&mut context, Order::Post, true)
        .raise(&mut None, None).err();
    // TODO: fix current on error?
    let validated = Context { root: core::marker::PhantomData::<Root>, args, current: context.current };
    (validated, result)
}

struct Validated<Root> {
    root: core::marker::PhantomData<Root>,
    args: &'static [Argument],
}

impl<Root: IsMeta> Context<Root> {
    fn validated(&self) -> Validated<Root> {
        Validated {
            args: &self.args[0..self.current],
            root: core::marker::PhantomData,
        }
    }
}

struct SubcommandIter<'v, Root: FromArgs> {
    args: &'v Validated<Root>,
    current: usize,
}

impl<'v, Root: FromArgs> Iterator for SubcommandIter<'v, Root> {
    type Item = Root::Subcommands;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

trait FromArgs {
    type Subcommands: FromArgs;
}

impl<Root: FromArgs> Validated<Root> {
    fn parse(&self) -> (Root, SubcommandIter<Root::Subcommands>) {
        todo!()
    }
}
