use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn parse(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "BO_" => println!("Found Message Object"),
            _ => continue
        }
    }

    Ok(())
}
