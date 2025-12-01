use std::{env, fs};

use can_dbc::Dbc;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: 'cli_app test   --- prints a test text");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "test" => {
            _ = parse_dbc_file("resources/example.dbc");
        }
        _ => {
            eprintln!("Unknown command: {command}");
            println!("Usage: 'cli_app test   --- prints a test text");
        }
    }
}

fn parse_dbc_file(file_path: &str) {
    let data = fs::read_to_string(file_path).expect("Unable to read input file");
    let dbc = Dbc::try_from(data.as_str()).expect("Failed to parse dbc file");
    println!("{:?}", dbc.messages);
}
