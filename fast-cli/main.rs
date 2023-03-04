// new crate from here

// TODO: may not be the best idea to use Display for usage.
impl std::fmt::Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "  {}", self.description)?;

        if !self.values.is_empty() {
            writeln!(f)?;
            writeln!(f, "Arguments:")?;
            for Value { name, description, .. } in self.values {
                writeln!(f, "  {}  {}", name, description)?;
            }
        }

        if !self.flags.is_empty() {
            writeln!(f)?;
            writeln!(f, "Flags:")?;
            for Flag { aliases, description, .. } in self.flags {
                writeln!(f, "  {}  {}", aliases.join(", "), description)?;
            }
        }

        if !self.subcommands.is_empty() {
            writeln!(f)?;
            writeln!(f, "Subcommands:")?;
            for Subcommand { aliases, description, .. } in self.subcommands {
                writeln!(f, "  {}  {}", aliases.join(", "), description)?;
            }
        }

        Ok(())
    }
}

impl Context {
    // should get moved to fast-cli
    fn handle(self, error: ValidationError) -> ! {
        use crate::ValidationError::*;
        match error {
            Unknown => {
                eprintln!("error: unknown argument `{}`", self.args[self.current]);
            }
            Invalid => {
                eprintln!("error: invalid argument `{}`", self.args[self.current]);
            }
            Repeated => {
                eprintln!("error: repeated argument `{}`", self.args[self.current]);
            }
            Missing => {
                eprintln!("error: missing argument");
            }
        }
        eprintln!();

        // tree

        for (meta, expected) in self.expected().iter().rev() {
            eprintln!("{meta:?}");
            eprintln!("{expected:?}");
            eprintln!();
        }

        std::panic::set_hook(Box::new(|_| {}));
        panic!()
    }

    fn validate<Root: Validate>(args: &'static [Argument]) -> std::result::Result<Validated<Root>, std::convert::Infallible> {
        let mut context = Context { args, current: 0, ast: vec![], states: vec![], states_ref: vec![] };
        let result = Root::validate(Order::Post, &mut context); // TODO: binary and subcommand skip

        if let Err(error) = result {
            context.handle(error)
        }

        // if context.next().is_some() {
        //     context.back();
        //     context.handle(ValidationError::Unknown)
        // }

        // Root::META.usage();
        println!("{}", Root::META);

        // println!("{}", Stack(context.stack));

        Ok(Validated {
            args: context.args,
            root: std::marker::PhantomData,
        })
    }
}

// back to caclap
