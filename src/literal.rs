use core::fmt::{Debug, Display, Formatter};
use alloc::string::{String, ToString};
use crate::{Context, Order, Validate};
use crate::error::{Error, Trace, Unknown};

#[derive(Debug)]
enum LiteralError {
    Missing(&'static Literal),
    Invalid { expected: String },
}

// #[cfg(feature = "std")]
// impl Error for LiteralError {}

impl From<LiteralError> for Trace {
    fn from(value: LiteralError) -> Self {
        use crate::{repeat::{Repeat::Required, Required::Exactly}};
        match value {
            LiteralError::Missing(literal) => Error::Missing(literal.into(), Required(Exactly(1))),
            LiteralError::Invalid { expected } => Error::Invalid { expected },
        }.into()
    }
}

pub struct Literal {
    pub name: &'static str,
    validate: fn(context: &mut Context, order: Order, last: bool) -> Result<(), LiteralError>,
}

impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Literal")
            .field("name", &self.name)
            .finish()
    }
}

impl Validate for Literal {
    fn validate(&self, context: &mut Context, order: Order, last: bool) -> Result<Option<Unknown>, Trace> {
        (self.validate)(context, order, last)?;
        Ok(None)
    }
}

pub trait FromStr {
    const LITERAL: Literal;
}

fn wrap_from_str<E, L>(context: &mut Context, _: Order, _: bool) -> Result<(), LiteralError>
    where E: Display, L: FromStr + core::str::FromStr<Err=E> {
    if let Some(token) = context.next() {
        if let Err(e) = token.parse::<L>() {
            Err(LiteralError::Invalid { expected: e.to_string() })
        } else {
            Ok(())
        }
    } else {
        Err(LiteralError::Missing(&L::LITERAL))
    }
}

macro_rules! literal {
    ($ty:ty) => {
        impl FromStr for $ty {
            const LITERAL: Literal = Literal {
                name: stringify!($ty),
                validate: wrap_from_str::<<$ty as core::str::FromStr>::Err, $ty>
            };
        }
    };
    ($first:ty, $($other:ty),+) => {
        literal!($first);
        literal!($($other),+);
    }
}

literal!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, char);
literal!(String);
