use std::error::Error;
use std::fs;
pub struct Config {
   pub query: String,
   pub filename: String,
}
impl Config {
   pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enougth args");
        }
        let query = args[1].to_string();
        let filename = args[2].to_string();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
}