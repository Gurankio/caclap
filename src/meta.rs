use crate::repeat::{Optional, Repeat};
use crate::error::{Trace, Unknown};
use crate::{Context, Order, Validate, Validator};

mod validate;

pub use validate::MetaValidate;

#[derive(Debug)]
pub struct Flag {
    pub aliases: &'static [&'static str],
    pub repeat: Optional,
    pub validator: Validator,
}

impl Validate for Flag {
    fn validate(&self, context: &mut Context, order: Order, last: bool) -> Result<Option<Unknown>, Trace> {
        self.validator.validate(context, order, last)
    }
}

#[derive(Debug)]
pub struct Value {
    pub name: &'static str,
    pub validator: Validator,
}

impl Validate for Value {
    fn validate(&self, context: &mut Context, order: Order, last: bool) -> Result<Option<Unknown>, Trace> {
        self.validator.validate(context, order, last)
    }
}

#[derive(Debug)]
pub struct Subcommand {
    pub aliases: &'static [&'static str],
    pub repeat: Repeat,
    pub validator: Validator,
}

impl Validate for Subcommand {
    fn validate(&self, context: &mut Context, order: Order, last: bool) -> Result<Option<Unknown>, Trace> {
        self.validator.validate(context, order, last)
    }
}

pub struct Meta {
    pub name: &'static str,
    pub flags: &'static [Flag],
    pub values: &'static [Value],
    pub subcommands: &'static [Subcommand],
    pub validate: fn(context: &mut Context, order: Order, last: bool) -> Result<Option<Unknown>, Trace>,
}

impl core::fmt::Debug for Meta {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Meta")
                .field("name", &self.name)
                .field("flags", &self.flags)
                .field("values", &self.values)
                .field("subcommands", &self.subcommands)
                .finish()
        } else {
            f.debug_struct("Meta")
                .field("name", &self.name)
                .finish()
        }
    }
}

impl Validate for Meta {
    fn validate(&self, context: &mut Context, order: Order, last: bool) -> Result<Option<Unknown>, Trace> {
        (self.validate)(context, order, last)
    }
}

pub trait IsMeta {
    const META: Meta;
}
