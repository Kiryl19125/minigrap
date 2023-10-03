use std::env;
use std::process;
use colored::*;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem with parsing arguments: {err}");
        process::exit(1);
    });

    println!("Serach for {}", config.query.as_str().green());
    println!("In file {}", config.file_path.as_str().green());
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
