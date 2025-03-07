mod config;

use config::Config;
use std::{env, error::Error, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    // let Config { query, file_path } = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        exit(1);
    });

    run(&config).unwrap_or_else(|err| {
        println!("Application error: {err}");
        exit(1);
    });
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let Config { query, file_path } = &config;

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n {contents}");

    Ok(())
}
