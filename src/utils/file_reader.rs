use anyhow::Result;
use std::{fs, io::Read};

pub fn read_text_from_file(day: &str) -> Result<String> {
    let base_path = "input/";

    let file_path = format!("{}{}", base_path, day);

    let mut file = fs::File::open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
