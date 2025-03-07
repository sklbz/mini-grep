mod config;

use config::Config;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // let Config { query, file_path } = parse_config(&args);
    let Config { query, file_path } = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {query} in {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
