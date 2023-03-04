# Extra Token

    error: unexpected argument `test` found
    
           target/debug/sample-user
             > multiple
             |     simple
             |       > (value): ComplexValue
             |       |     12: u32
             |       |     --option 1
             |       |     --default
             |       |         1: u32
             |       |         2: u32
             |       |         3: u32
             |       |     --boolean
             |       |     --counter
             |       |     ^^^^^^^^^ takes no arguments
             |       |
             |       expected one of `--boolean`, ...
             |
             expected a `simple`
    
    help: should read
    
        target/debug/sample-user help Multiple
        target/debug/sample-user help ComplexValue

# Typo

    error: unexpected argument `simpla` found

           target/debug/sample-user
               multiple
                   simple
                       (value): ComplexValue
                           12: u32
                           --counter
                           --counter
                           --boolean
                           --counter

    help: did you mean `simple`?

            target/debug/sample-user
                multiple
                    simple
                        (value): ComplexValue
                            12: u32
                            --counter
                            --counter
                            --boolean
                            --counter
                    simple
                        ...

# Out Of Tokens

    error: out of arguments

           target/debug/sample-user
               multiple
                   simple
                       (value: ComplexValue)
                           value: u32
                           ^^^^^ missing argument

    help: should read
    
        target/debug/sample-user help ComplexValue

# Invalid

    error: invalid token `12.3`

           target/debug/sample-user
               multiple
                   simple
                       (value): ComplexValue
                           12.3
                           ^^^^ invalid argument
                                expected an unsigned integer

# Dirty Flag

    error: overwriting previous flag

           target/debug/sample-user
               multiple
                   simple
                       (value): ComplexValue
                           12: u32
                         > --boolean
                         | --counter
                         > --no-boolean
                         |
                         here

    help: if this is intended add `--no-duplicate-error` flag at the root

           target/debug/sample-user
               --no-duplicate-error
               multiple
                   simple
                       (value): ComplexValue
                           12: u32
                           --boolean
                           --counter
                           --no-boolean


# Help
## Root

    $ that_string help
    $ that_string --help
    A simple to use, efficient, full-featured, and Context-Aware Command Line Argument Parser

    Usage: 
      that_string [OPTIONS] COMMAND 
     
    Options:
      -h, --help                print help
      -V, --version             print version
          --no-duplicate-error  suppresses errors when overwriting a previous flag

    Commands:
      help         print this message or the help of the given subcommand(s)
      version      print version
      completions  manage completions
      single       single command example
      multiple     multiple commands example

## Single / Multiple

    $ that_string help single
    $ that_string help multiple
    $ that_string single --help
    $ that_string single --multiple
    Single's description

    Usage: 
      that_string single [OPTIONS] COMMAND 
     
    Options:
      -h, --help                 print help
      -d, --default <EnumValue>  the default thing

    Commands:
      simple       simple subcommand

## Simple

    $ that_string help single simple
    $ that_string help multiple simple
    $ that_string single simple --help
    $ that_string multiple simple --help
    Simple's description

    Usage: 
      that_string single simple [OPTIONS] <ComplexValue> 
     
    Options:
      -h, --help                 print help
      -d, --default <EnumValue>  the default thing

## ComplexValue?

    $ that_string help ComplexValue
    ComplexValue's description

    Usage: 
      <u32> [OPTIONS]

    Arguments:
      <u32>  the value thing

    Options:
      -c, --counter..      the counter thing
      -d, --default <f32>  the default thing      [default: 42]
      -o, --option <f32>   the optional thing
          --no-option      the no optional thing
      -b, --boolean        the true thing
          --no-boolean     the false thing
