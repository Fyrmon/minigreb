use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (_query, file_name) = parse_config(&args);

    let file_content = fs::read_to_string(file_name)
        .expect("File cant open/read");

    println!("File contents: {file_content}")

}

fn parse_config(args: &[String]) -> (&str, &str)
{
    (&args[1], &args[2])
}
