use std::env;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum CmdOptionValueType {
    String,
    Number,
    Float,
    NoValue,
}

#[derive(Debug)]
enum CmdArgumentValue {
    String(String),
    Number(i32),
    Float(f32),
    NoValue
}

#[derive(Debug)]
struct CmdArgument {
    option: String,
    value: CmdArgumentValue,
}

#[derive(Debug)]
struct CmdLineOption<'a> {
    shortform: &'a str,
    longform: &'a str,
    description: &'a str,
    value_type: CmdOptionValueType,
}

pub struct Commander<'a> {
    options: Vec<CmdLineOption<'a>>,
    args: HashMap<String, CmdArgument>,
}

/*
    TODO:

    - Write a method to retrieve an option's value (if it exists) (otherwise None)
*/

impl<'a> Commander<'a> {
    ///
    /// Create's a new instance of the Commander struct
    pub fn new() -> Commander<'a> {
        Commander {
            options: Vec::new(),
            args: HashMap::new(),
        }
    }

    ///
    /// You should call this method as the last call as part of initialisation of supported options.
    /// 
    /// # Examples
    /// ```
    /// use commander::{Commander, CmdOptionValueType};
    /// 
    /// let mut cmd = Commander::new();
    /// cmd.add_option("v", "version", "Show the version of this application", CmdOptionValueType::NoValue)
    ///     .add_option("h", "help", "Show this help", CmdOptionValueType::NoValue)
    ///     .add_option("if", "input", "File to use as input", CmdOptionValueType::String)
    ///     .init();
    /// ```
    pub fn init(&mut self) {
        let args = env::args().collect::<Vec<String>>();
        let mut current_arg: CmdArgument;

        // store the first element as the process launch executable
        self.args.insert(String::from("__exec__"), CmdArgument { option: "__exec__".to_string(), value: CmdArgumentValue::String(args[0].clone())});

        let mut iter = args.iter().skip(1);

        while let Some(arg) = iter.next() {
            let shortform = arg.starts_with("-");
            let longform = arg.starts_with("--");
            let value = {
                if longform {
                    &arg[2..]
                } else if shortform {
                    &arg[1..]
                } else {
                    arg
                }
            };

            if longform || shortform {
                let o = self.get_supported_option(value, longform);
                match o {
                    Some(option) => {
                        current_arg = CmdArgument {
                            option: option.shortform.to_string(),
                            value: CmdArgumentValue::NoValue,
                        };

                        // if we are expecting a value in the next element...
                        if option.value_type != CmdOptionValueType::NoValue {
                            if let Some(v) = iter.next() {
                                current_arg.value = match option.value_type {
                                    CmdOptionValueType::String => CmdArgumentValue::String(v.to_string()),
                                    CmdOptionValueType::Number => CmdArgumentValue::Number(v.parse().unwrap()),
                                    CmdOptionValueType::Float => CmdArgumentValue::Float(v.parse().unwrap()),
                                    _ => unreachable!(),
                                }
                            }
                        }

                        self.args.insert(option.shortform.to_string(), CmdArgument { 
                            option: current_arg.option,
                            value: current_arg.value,
                        });
                        // eprint!("OPTION: {:?}\n", option);
                        // eprint!("[OK] O({}): ", if longform { "L" } else { "S" });
                    },
                    None => eprintln!("[BAD] O({}): {}", if longform { "L" } else { "S" }, value),
                } 
            } else {
                eprintln!("[BAD?] V: {}", value);
            }
        }

        eprintln!("INIT COMPLETE: {:?}", self.args);
    }

    ///
    /// Add a supported option. All added options will be checked for when the Commander finally initialises with
    /// the provided command line arguments
    pub fn add_option(&mut self, shortform: &'a str, longform: &'a str, description: &'a str, value_type: CmdOptionValueType) -> &mut Self{
        let option = CmdLineOption {
            shortform,
            longform,
            description,
            value_type
        };

        self.options.push(option);
        self.options.sort_by(|a, b| a.shortform.cmp(&b.shortform) );

        self
    }

    ///
    /// Returns the number of supported options that have been added to this instance of Commander
    pub fn option_count(&self) -> usize {
        self.options.len()
    }

    ///
    /// Returns the path and filename of the calling executable of the current process
    // pub fn executable(&'a self) -> &'a String {
    //     // &self.args[0]
    // }

    ///
    /// Returns a string which contains a formatted output of available options and descriptions
    pub fn help(&self) -> String {
        let mut output = String::from("Options available:\n");

        for option in self.options.iter() {
            output.push_str(&format!("\t--{}, -{}", option.longform, option.shortform));
            match option.value_type {
                CmdOptionValueType::String => output.push_str(&format!("\t\t[string]")),
                CmdOptionValueType::Float => output.push_str(&format!("\t\t[Float]")),
                CmdOptionValueType::Number => output.push_str(&format!("\t\t[Number]")),
                CmdOptionValueType::NoValue => output.push_str("\t\t[no paramater]")
            }
            output.push_str(&format!("\t\t{}\n", 
                        option.description.to_string()));
        }

        output
    }
    
    //
    // Private
    // Checks if the provided option is supported by this instance of Commander
    fn get_supported_option(&self, option: &'a str, is_longform: bool) -> Option<&'a CmdLineOption> {
        let result = self.options.iter().find(|o| {
            (!is_longform && o.shortform == option) || (is_longform && o.longform == option)
        });

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_1_option() {
        let mut cmd = Commander::new();

        cmd.add_option("1", "one", "Option 1 description", CmdOptionValueType::NoValue);
        assert_eq!(1, cmd.option_count());
    }
    #[test]
    fn test_add_2_options() {
        let mut cmd = Commander::new();

        cmd.add_option("v", "version", "Prints the version of the application", CmdOptionValueType::NoValue)
            .add_option("h", "help", "Prints this help", CmdOptionValueType::NoValue);
        assert_eq!(cmd.option_count(), 2);
    }

    #[test]
    #[ignore]
    fn test_help() {
        let mut cmd = Commander::new();
        let expectation = "Options available:\n\t--h, -help\t\tPrints this help\n\t--v, -version\t\tPrint the version\n";

        cmd.add_option("v", "version", "Print the version", CmdOptionValueType::NoValue)
            .add_option("h", "help", "Prints this help", CmdOptionValueType::NoValue);
        assert_eq!(expectation, cmd.help());
    }
}