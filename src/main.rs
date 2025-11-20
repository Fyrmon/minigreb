use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigreb::search_case_sensitive;
use minigreb::search_case_insensitive;

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("App error: {e}");
        process::exit(1);
    }

}

struct Config
{
    query: String,
    file_name: String,
    ignore_case: bool
}

impl Config 
{
    fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str>
    {
        if args.len() < 3 {
            return Err("Not enough arguments for program to run");
        }

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            query:args[1].clone(), 
            file_name: args[2].clone(),
            ignore_case
        })
    }    
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let file_contents = fs::read_to_string(config.file_name)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &file_contents)
    } else {
        search_case_sensitive(&config.query, &file_contents)
    };

        for line in result {
            println!("{line}");
        }

    Ok(())
}

