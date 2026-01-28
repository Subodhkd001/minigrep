use core::panic;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err: &str| {
        println!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Query {} and file path is {}",
        config.query, config.filename
    );
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3{
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
        
    }
}
