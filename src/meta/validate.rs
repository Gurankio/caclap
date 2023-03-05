use alloc::vec::Vec;
use crate::{Context, Order, Validate};
use crate::error::{Error, Raise, Trace, Unknown};
use crate::meta::{Flag, IsMeta, Subcommand};
use crate::repeat::{Optional, Repeat};

pub trait MetaValidate {
    fn validate(context: &mut Context, order: Order, last: bool) -> Result<Option<Unknown>, Trace>;
}

fn flags(context: &mut Context, flags: &'static [Flag]) -> Result<Option<Unknown>, Trace> {
    let mut output = None;
    let mut matched: Vec<Optional> = flags.iter()
        .map(|flag| flag.repeat)
        .collect();

    while let Some(next) = context.next() {
        let flag = flags
            .iter()
            .enumerate()
            .find(|(_, flags)| flags.aliases.contains(next));

        if flag.is_none() {
            context.back();
            break;
        }

        let (index, flag) = flag.unwrap();

        matched[index] = match matched[index].matched() {
            Some(value) => value,
            None => return Err(Error::Repeated(flag.into()).into()),
        };

        flag.validate(context, Order::Pre, false)
            .raise(&mut output, Some(flag.into()))?;
    }

    if context.next().is_some() {
        let valid: Vec<&Flag> = flags.iter()
            .enumerate()
            .filter(|(i, _)| !matched[*i].exhausted())
            .map(|(_, f)| f)
            .collect();

        if !valid.is_empty() {
            output = match output {
                Some(Unknown::Flag { valid: other }) => Some(Unknown::Flag { valid: [valid, other].concat() }),
                _ => Some(Unknown::Flag { valid }),
            }
        }

        context.back();
    }

    Ok(output)
}

fn subcommands(context: &mut Context, subcommands: &'static [Subcommand], last: bool) -> Result<Option<Unknown>, Trace> {
    let mut output = None;
    let mut matched: Vec<Repeat> = subcommands.iter()
        .map(|subcommand| subcommand.repeat)
        .collect();

    while let Some(next) = context.next() {
        let subcommand = subcommands
            .iter()
            .enumerate()
            .find(|(_, subcommand)| subcommand.aliases.contains(next));

        if subcommand.is_none() {
            context.back();
            break;
        }

        let (index, subcommand) = subcommand.unwrap();

        matched[index] = match matched[index].matched() {
            Some(value) => value,
            None => return Err(Error::Repeated(subcommand.into()).into()),
        };

        subcommand.validate(context, Order::Post, last)
            .raise(&mut output, Some(subcommand.into()))?;
    }

    if let Some((index, repeat)) = matched.iter()
        .enumerate()
        .find(|(_, repeat)| repeat.needed()) {
        return Err(Error::Missing((&subcommands[index]).into(), *repeat).into());
    }

    if last && context.next().is_some() {
        let valid: Vec<&Subcommand> = subcommands.iter()
            .enumerate()
            .filter(|(i, _)| !matched[*i].exhausted())
            .map(|(_, f)| f)
            .collect();

        if !valid.is_empty() {
            output = match output {
                Some(Unknown::Subcommand { valid: other }) => Some(Unknown::Subcommand { valid: [valid, other].concat() }),
                _ => Some(Unknown::Subcommand { valid }),
            }
        }

        context.back();
    }

    Ok(output)
}

impl<M: IsMeta> MetaValidate for M {
    fn validate(context: &mut Context, order: Order, last: bool) -> Result<Option<Unknown>, Trace> {
        let meta = M::META;
        let mut output = None;

        match order {
            Order::Pre => {
                for value in meta.values {
                    value.validate(context, Order::Pre, false)
                        .raise(&mut output, Some(value.into()))?;
                }
                flags(context, meta.flags)
                    .raise(&mut output, None)?;
                // subcommands(context, meta.subcommands, last)
                //     .raise(&mut output, None)?;
            }
            Order::Post => {
                flags(context, meta.flags)
                    .raise(&mut output, None)?;
                for value in meta.values {
                    value.validate(context, Order::Pre, false)
                        .raise(&mut output, Some(value.into()))?;
                }
                subcommands(context, meta.subcommands, last)
                    .raise(&mut output, None)?;
            }
        }

        Ok(output)
    }
}
