use std::env;
use std::fs;
use std::process;

use std::error::Error;
use minigrep::search;


struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {

        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let file_path = &args[2];

    Config { query: query.to_string(), file_path: file_path.to_string() }
}

// cargo run -- bog poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);


    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }


}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    for line in search(&config.query, &contents) {
        println!("Search line :{line}");
    }

    Ok(())
}