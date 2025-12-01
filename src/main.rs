use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: 'cli_app test   --- prints a test text");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "test" => {
            println!("This is test for CLI application");
        }
        _ => {
            eprintln!("Unknown command: {command}");
            println!("Usage: 'cli_app test   --- prints a test text");
        }
    }
}
