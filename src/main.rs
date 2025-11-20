use minigreb::search_case_insensitive;
use minigreb::search_case_sensitive;
use std::{env, error::Error, fs, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("App error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_name: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a query string"),
        };

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get file path"),
        };

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_name,
            ignore_case,
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
