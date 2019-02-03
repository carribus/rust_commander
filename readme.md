# rust_commander
The rust_commander crate provides a structured way of setting up a command line parser of options
and type specifiers per option. Any arguments passed via the command line that are not registered
as options are ignored.

## Example
    
```
fn main() {
    let mut cmd = Commander::new();

    cmd.add_option("v", "version", "Show the version of this application", CmdOptionValueType::NoValue)
        .add_option("h", "help", "Show this help", CmdOptionValueType::NoValue)
        .add_option("if", "input", "File to use as input", CmdOptionValueType::String)
        .add_option("c", "count", "Amount of times to do something", CmdOptionValueType::Number)
        .add_option("b", "balance", "Amount of money in your bank account", CmdOptionValueType::Float)
        .init();

    if cmd.arg_count() == 1 {
        println!("{}", cmd.help());
    } else {
        let iter = cmd.arguments();
        for k in iter {
            dbg!(k);
        }
    }
}
```

Once the command line arguments are parsed, you can fetch the provided arguments through either an iterator
from `.arguments()` or by specifically fetching the value of each argument of interest via the `get_number_option()`,
`get_float_option()` or `get_string_option()` methods.