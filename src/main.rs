use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let file_content = fs::read_to_string(config.file_name)
        .expect("File cant open/read");

    println!("File contents: {file_content}")

}

struct Config
{
    _query: String,
    file_name: String
}

impl Config 
{
    fn build(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3 {
            return Err("Not enough arguments for program to run");
        }

        Ok(Config { 
            _query:args[1].clone(), 
            file_name: args[2].clone()
        })
    }    
}

