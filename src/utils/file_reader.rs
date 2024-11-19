use anyhow::{Context, Result};
use std::{fs, io::Read};

pub fn read_text_from_file(day: &str) -> Result<String> {
    let base_path = "input/day";

    let file_path = format!("{}{}.txt", base_path, day);

    let mut file = fs::File::open(&file_path)
        .with_context(|| format!("failed to read path: {}", file_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
