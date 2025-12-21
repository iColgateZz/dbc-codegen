use can_dbc::{Dbc, DbcError};
use dbc_codegen::DbcFile;
use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
};

const FILEPATHS: [&str; 4] = [
    "resources/example.dbc",
    "resources/BMW-PHEV-HV-Battery.dbc",
    "resources/Kangoo.dbc",
    "resources/VW-GTE-HV-Battery.dbc",
];
const MAX_INDEX: usize = FILEPATHS.len();

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "test" => match parse_dbc_file(FILEPATHS[0]) {
            Ok(dbc) => {
                if let Err(e) = generate_code(dbc) {
                    eprintln!("Error generating code: {e}");
                }
            }
            Err(e) => eprintln!("{:?}", e),
        },
        "test-ir" => {
            if args.len() < 3 {
                println!("Add index as well!");
                print_usage();
                return;
            }
            let index: usize = args[2].parse().expect("Index must be number!");
            let filepath = FILEPATHS[index];
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
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  cargo run test");
    println!("  cargo run test-ir <index 0..{MAX_INDEX}>");
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
