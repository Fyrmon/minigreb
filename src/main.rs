use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _query = &args[1];
    let file_name = &args[2];

    let file_content = fs::read_to_string(file_name)
        .expect("File cant open/read");

    println!("File contents: {file_content}")



}
