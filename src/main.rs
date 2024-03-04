use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file!");
    println!("Content in the file: \n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    println!("Query: {query}");
    println!("From the file: {file_path}");
    Config { query, file_path }
}
