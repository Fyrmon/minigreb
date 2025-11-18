use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    let file_content = fs::read_to_string(config.file_name)
        .expect("File cant open/read");

    println!("File contents: {file_content}")

}

struct Config
{
    _query: String,
    file_name: String
}

fn parse_config(args: &[String]) -> Config
{
    Config { 
        _query:args[1].clone(), 
        file_name: args[2].clone()
    }
}
