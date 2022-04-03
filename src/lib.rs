use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
}

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String
}

impl<'a> Config<'a> {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("Not enough arguments")
        } else {
            Ok(Config { query: &args[1], filename: &args[2] })
        }
    }
}
