use std::{error::Error, fs};

#[allow(dead_code)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

#[allow(dead_code)]
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let Config { query, file_path } = &self;

        println!("Searching for {query}");
        println!("In file {file_path}");

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        println!("With text:\n {contents}");

        Ok(())
    }
}

#[allow(dead_code)]
pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
