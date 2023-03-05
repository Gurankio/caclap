// #![no_std]
#![feature(core_intrinsics)]

extern crate alloc;

mod new;

mod literal;
mod repeat;
mod error;
mod meta;

#[cfg(test)]
mod tests;

use std::marker::PhantomData;
use std::ops::Sub;
use literal::Literal;
use error::{Trace, Unknown, Raise};
use meta::{IsMeta, Meta};
use crate::meta::Subcommand;

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
    root: PhantomData<Root>,
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

pub fn validate<Root: IsMeta>(args: &'static [Argument]) -> (Context<Root>, Option<Trace>) {
    let mut context = Context { root: PhantomData, args, current: 0 };
    let result = Root::META.validate(&mut context, Order::Post, true)
        .raise(&mut None, None).err();
    // TODO: fix current on error?
    let validated = Context { root: PhantomData::<Root>, args, current: context.current };
    (validated, result)
}

pub struct Validated {
    // root: core::marker::PhantomData<Root>,
    args: &'static [Argument],
}

impl<Root: IsMeta> Context<Root> {
    pub fn partial<T>(&self) -> &'static [Argument] {
        &self.args[0..self.current]
    }

    pub fn validated<T>(&self) -> Option<Validated> {
        if self.args.len() == self.current {
            Some(Validated {
                args: &self.args[0..self.current],
                // root: core::marker::PhantomData,
            })
        } else {
            None
        }
    }
}

pub trait FromArgs {
    fn from_args(validated: &mut Validated, order: Order) -> Self;
}

pub struct SubcommandIter<'v, Subcommands: FromArgs> {
    subcommands: PhantomData<Subcommands>,
    validated: &'v mut Validated,
    valid: &'static [Subcommand],
}

impl<'v, Subcommands: FromArgs> Iterator for SubcommandIter<'v, Subcommands> {
    type Item = Subcommands;

    fn next(&mut self) -> Option<Self::Item> {
        // if self.valid.iter().map(|s| s.aliases).flatten().find(self.validated.peek()).is_some() {
        //     return Some(Subcommands::from_args(self.validated, Order::Post));
        // } else {
        //     None
        // }
        todo!()
    }
}

pub trait HasSubcommands: IsMeta {
    type Subcommands: FromArgs;

    fn iter_subcommands(validated: &mut Validated) -> SubcommandIter<Self::Subcommands> {
        SubcommandIter { subcommands: PhantomData, validated, valid: Self::META.subcommands }
    }
}
