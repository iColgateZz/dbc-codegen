use can_dbc::Dbc;
use dbc_codegen::parse_dbc_file;
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: 'cli_app test   --- prints a test text");
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
            println!("Usage: 'cli_app test   --- prints a test text");
        }
    }
}

fn generate_code(dbc: Dbc) -> std::io::Result<()> {
    let output_file = File::create("test.rs")?;
    let mut writer = BufWriter::new(output_file);

    writeln!(writer, "// Generated test file")?;
    writeln!(writer, "//{:?}", dbc.messages)?;

    Ok(())
}
