use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

// Writes to local file.
pub fn save_to_file(file_path: &str, data: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(file_path)?;
    writeln!(file, "{}", data)?;
    Ok(())
}

pub fn read_from_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn clear_file(file_path: &str) -> io::Result<()> {
    File::create(file_path)?;
    Ok(())
}

