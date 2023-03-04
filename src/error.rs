use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;
use crate::literal::Literal;
use crate::meta::{Flag, Subcommand, Value};
use crate::repeat::Repeat;

#[derive(Debug, Clone)]
pub enum Unknown {
    Flag { valid: Vec<&'static Flag> },
    Subcommand { valid: Vec<&'static Subcommand> },
}

/// anything that could have not been not fully matched
#[derive(Debug)]
pub enum Partial {
    Flag(&'static Flag),
    Value(&'static Value),
    Subcommand(&'static Subcommand),
    Literal(&'static Literal),
}

impl From<&'static Flag> for Partial {
    fn from(flag: &'static Flag) -> Self {
        Self::Flag(flag)
    }
}

impl From<&'static Value> for Partial {
    fn from(value: &'static Value) -> Self {
        Self::Value(value)
    }
}

impl From<&'static Subcommand> for Partial {
    fn from(subcommand: &'static Subcommand) -> Self {
        Self::Subcommand(subcommand)
    }
}

impl From<&'static Literal> for Partial {
    fn from(literal: &'static Literal) -> Self {
        Self::Literal(literal)
    }
}

#[derive(Debug)]
pub enum Error {
    /// must be pushed when forwarding an error adding context
    Partial(Partial),

    /// an entity has been repeated more than what is allowed
    Repeated(Partial),

    /// a required entity has not been found
    Missing(Partial, Repeat),

    /// an unknown argument has been found
    Unknown(Unknown),

    /// either it is unknown or the other
    Either(Unknown, Box<Trace>),

    /// an invalid argument was found, i.e. a letter in a base10 number
    Invalid {
        /// usually the source error's to_string
        expected: String,
    },
}

#[derive(Debug)]
pub struct Trace {
    source: Option<Box<Trace>>,
    error: Error,
}

impl From<Error> for Trace {
    fn from(error: Error) -> Trace {
        Trace { source: None, error }
    }
}

impl Trace {
    fn context(self, partial: Partial) -> Trace {
        Trace {
            source: Some(Box::new(self)),
            error: Error::Partial(partial),
        }
    }
}

impl<'a> IntoIterator for &'a Trace {
    type Item = &'a Error;
    type IntoIter = TraceIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TraceIter { current: Some(self) }
    }
}

pub struct TraceIter<'a> {
    current: Option<&'a Trace>,
}

impl<'a> Iterator for TraceIter<'a> {
    type Item = &'a Error;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            self.current = current.source.as_deref();
            Some(&current.error)
        } else {
            None
        }
    }
}

pub trait Raise {
    fn raise(self, potential: &mut Option<Unknown>, partial: Option<Partial>) -> Result<(), Trace>;
}

impl Raise for Result<Option<Unknown>, Trace> {
    fn raise(self, potential: &mut Option<Unknown>, partial: Option<Partial>) -> Result<(), Trace> {
        match (&potential, self) {
            (_, Ok(pot)) => {
                // overriding the old potential
                *potential = pot;
                Ok(())
            }
            (None, Err(err)) => {
                if let Some(partial) = partial {
                    Err(err.context(partial))
                } else {
                    Err(err)
                }
            }
            (Some(old), Err(err)) => {
                let trace = Error::Either(
                    old.clone(),
                    (if let Some(partial) = partial { err.context(partial) } else { err }).into(),
                ).into();

                *potential = None;
                Err(trace)
            }
        }
    }
}
