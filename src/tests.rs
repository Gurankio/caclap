#![allow(dead_code)]

use crate::{FromArgs, HasSubcommands, Order, Validated, Validator};
use crate::literal::FromStr;
use crate::meta::{Flag, IsMeta, Meta, Subcommand, Value, MetaValidate};
use crate::repeat::{Optional, Repeat, Required};

struct Simple {
    a: u32,
    b: f64,
    c: Option<usize>,
}

impl IsMeta for Simple {
    const META: Meta = Meta {
        name: "Simple",
        flags: &[
            Flag {
                aliases: &["-c", "--c"],
                repeat: Optional::AtMost(1),
                validator: Validator::Literal(&usize::LITERAL),
            }
        ],
        values: &[
            Value {
                name: "a",
                validator: Validator::Literal(&u32::LITERAL),
            },
            Value {
                name: "b",
                validator: Validator::Literal(&f64::LITERAL),
            },
        ],
        subcommands: &[
            Subcommand {
                aliases: &["x"],
                repeat: Repeat::Required(Required::Exactly(1)),
                validator: Validator::Literal(&usize::LITERAL),
            }
        ],
        validate: Simple::validate,
    };
}

impl FromArgs for Simple {
    fn from_args(validated: &mut Validated, order: Order) -> Self {
        let (a, b, c);
        match order {
            Order::Pre => {
                a = u32::from_args(validated, Order::Pre);
                b = f64::from_args(validated, Order::Pre);
                c = None;
            }
            Order::Post => {
                c = None;
                a = u32::from_args(validated, Order::Pre);
                b = f64::from_args(validated, Order::Pre);
                // subcommands?
            }
        }
        Self { a, b, c }
    }
}

enum SimpleSubcommands {
    X(usize)
}

impl FromArgs for SimpleSubcommands {
    fn from_args(validated: &mut Validated, order: Order) -> Self {
        todo!()
    }
}

impl HasSubcommands for Simple {
    type Subcommands = SimpleSubcommands;
}

struct Nested {
    simple: Simple,
    d: Option<Simple>,
}

impl IsMeta for Nested {
    const META: Meta = Meta {
        name: "Nested",
        flags: &[
            Flag {
                aliases: &["-d", "--d"],
                repeat: Optional::AtMost(1),
                validator: Validator::Meta(&Simple::META),
            }
        ],
        values: &[
            Value {
                name: "simple",
                validator: Validator::Meta(&Simple::META),
            }
        ],
        subcommands: &[
            Subcommand {
                aliases: &["y"],
                repeat: Repeat::Required(Required::AtLeast(2)),
                validator: Validator::Meta(&Simple::META),
            }
        ],
        validate: Nested::validate,
    };
}

#[test]
fn simple() {
    static ARGS: [&str; 6] = ["--c", "12", "1", "2", "x", "12"];
    if let Some(err) = crate::validate::<Simple>(&ARGS).1 {
        for item in &err {
            println!("{item:#?}");
        }
        panic!()
    }
}

#[test]
fn nested() {
    static ARGS: [&str; 27] = [
        "-d", "1", "2", "--c", "12", "x", "12",
        "1", "2", "--c", "12", "x", "12",
        "y", "--c", "12", "1", "2", "x", "12",
        "y", "--c", "12", "1", "2", "x", "12",
    ];
    if let (context, Some(err)) = crate::validate::<Nested>(&ARGS) {
        for item in &err {
            println!("{item:#?}");
        }
        panic!()
    }
}
