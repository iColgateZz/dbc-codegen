use can_dbc::{Dbc, DbcError};
use dbc_codegen::DbcFile;
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
    let filepaths = vec![
        "resources/example.dbc",
        "resources/BMW-PHEV-HV-Battery.dbc",
        "resources/Kangoo.dbc",
        "resources/VW-GTE-HV-Battery.dbc",
    ];

    match command.as_str() {
        "test" => match parse_dbc_file(filepaths[0]) {
            Ok(dbc) => {
                if let Err(e) = generate_code(dbc) {
                    eprintln!("Error generating code: {e}");
                }
            }
            Err(e) => eprintln!("{:?}", e),
        },
        "test-ir" => {
            let index: usize = args[2].parse().expect("");
            let filepath = filepaths[index];
            match parse_dbc_file(filepath) {
                Ok(dbc) => {
                    let ir = DbcFile::from(dbc);
                    println!("{:#?}", ir);
                }
                Err(e) => eprint!("{:?}", e),
            }
        }
        _ => {
            eprintln!("Unknown command: {command}");
            println!("Available commands: [ test, test-ir ]");
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
