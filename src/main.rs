use std::env;
use std::process;

use minigrep::Config;

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

    if let Err(e) = minigrep::run(config){
        println!("Application error : {e}");
        process::exit(1);
    }
    
}
