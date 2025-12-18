use std::fs;
use can_dbc::{Dbc, DbcError};

pub fn parse_dbc_file(file_path: &str) -> Result<Dbc, DbcError> {
    let data = fs::read_to_string(file_path).expect("Unable to read input file");
    return Dbc::try_from(data.as_str());
}
