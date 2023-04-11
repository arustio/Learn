use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn builld(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }  
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::builld(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    run(config);
}

fn run(config: Config) {
    let content = fs::read_to_string(config.file_path).expect("Something went wrong reading the file");
    print!("content: {}", content);
}