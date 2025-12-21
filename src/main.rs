use can_dbc::{Dbc, DbcError};
use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <command>");
        return;
    }

    let command = &args[1];
    let filepath = "resources/example.dbc";

    match command.as_str() {
        "test" => match parse_dbc_file(filepath) {
            Ok(dbc) => {
                if let Err(e) = generate_code(dbc) {
                    eprintln!("Error generating code: {e}");
                }
            }
            Err(e) => eprintln!("{:?}", e),
        },
        _ => {
            eprintln!("Unknown command: {command}");
            println!("Available commands: [ test ]");
        }
    }
}

pub fn parse_dbc_file(file_path: &str) -> std::result::Result<Dbc, DbcError> {
    let data = fs::read_to_string(file_path).expect("Unable to read input file");
    Dbc::try_from(data.as_str())
}

fn generate_code(dbc: Dbc) -> std::io::Result<()> {
    let output_file = File::create("test.rs")?;
    let mut writer = BufWriter::new(output_file);

    writeln!(writer, "// Generated test file")?;
    writeln!(writer, "//{:?}", dbc.messages)?;

    Ok(())
}
