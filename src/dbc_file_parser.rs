use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
struct MessageObject {
    message_id: u16,
    name: String,
    length: u8,
    transmitter_node: String
}

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
            "BO_" => {
                let message_object = MessageObject {
                    message_id: parts[1].parse::<u16>().unwrap(),
                    name: parts[2].replace(":", "").to_string(),
                    length: parts[3].parse::<u8>().unwrap(),
                    transmitter_node: parts[4].to_string()
                };
                println!("{:?}", message_object)
            }
            _ => continue,
        }
    }

    Ok(())
}
