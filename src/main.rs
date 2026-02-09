use can_dbc::Dbc as ParsedDbc;
use dbc_codegen::{DbcFile, codegen};
use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
};

const FILEPATHS: [&str; 5] = [
    "resources/example.dbc",
    "resources/BMW-PHEV-HV-Battery.dbc",
    "resources/Kangoo.dbc",
    "resources/VW-GTE-HV-Battery.dbc",
    "resources/simple.dbc",
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
        "test" => {
            let dbc = parse_dbc_file(FILEPATHS[0]);
            if let Err(e) = write_parsed_dbc(dbc) {
                eprintln!("Error generating code: {e}");
            }
        }
        "test-ir" => {
            if args.len() < 3 {
                println!("Add index as well!");
                print_usage();
                return;
            }
            let index = get_index(&args[2]);
            let dbc = parse_dbc_file(FILEPATHS[index]);
            let ir = DbcFile::from(dbc);
            println!("{:#?}", ir);
        }
        "gen" => {
            let index = get_index(&args[2]);
            let dbc = DbcFile::from(parse_dbc_file(FILEPATHS[index]));
            let generator = codegen::rust::RustGen::new();
            let code = generator.generate(&dbc.messages);
            let mut out = File::create("src/test.rs").unwrap();
            write!(out, "{}", code).unwrap();
        }
        _ => {
            eprintln!("Unknown command: {command}");
            print_usage();
        }
    }
}

fn get_index(arg: &str) -> usize {
    arg.parse().expect("Index must be number!")
}

fn print_usage() {
    println!("Usage:");
    println!("  cargo run test");
    println!("  cargo run test-ir <index 0..{MAX_INDEX}>");
}

pub fn parse_dbc_file(file_path: &str) -> ParsedDbc {
    let data = fs::read_to_string(file_path).expect("Unable to read input file");
    ParsedDbc::try_from(data.as_str()).unwrap()
}

fn write_parsed_dbc(dbc: ParsedDbc) -> std::io::Result<()> {
    let output_file = File::create("test.txt")?;
    let mut writer = BufWriter::new(output_file);

    writeln!(writer, "// Generated test file")?;
    writeln!(writer, "{:#?}", dbc.messages)?;

    Ok(())
}
