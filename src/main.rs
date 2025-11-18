use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigreb::search;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("App error: {e}");
        process::exit(1);
    }

}

struct Config
{
    query: String,
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
            query:args[1].clone(), 
            file_name: args[2].clone()
        })
    }    
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let file_contents = fs::read_to_string(config.file_name)?;

    for line in search(&config.query, &file_contents) {
        println!("{line}");
    }

    Ok(())
}

