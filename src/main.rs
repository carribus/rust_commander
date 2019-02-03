use commander::{Commander, CmdOptionValueType};


fn main() {
    let mut cmd = Commander::new();

    cmd.add_option("v", "version", "Show the version of this application", CmdOptionValueType::NoValue)
        .add_option("h", "help", "Show this help", CmdOptionValueType::NoValue)
        .add_option("if", "input", "File to use as input", CmdOptionValueType::String)
        .add_option("c", "count", "Amount of times to do something", CmdOptionValueType::Number)
        .add_option("b", "balance", "Amount of money in your bank account", CmdOptionValueType::Float)
        .init()
        ;

    if cmd.arg_count() == 1 {
        println!("{}", cmd.help());
    } else {
        let iter = cmd.arguments();
        for k in iter {
            dbg!(k);
        }
    }

}
