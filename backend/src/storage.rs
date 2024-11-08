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


#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::io;

    const TEST_FILE_PATH: &str = "test_storage.json";

    #[test]
    fn test_save_to_file() -> io::Result<()> {
        let data = "Hello, world!";

        save_to_file(TEST_FILE_PATH, data)?;

        let saved_data = fs::read_to_string(TEST_FILE_PATH)?;
        assert_eq!(saved_data, data);

        fs::remove_file(TEST_FILE_PATH)?;
        Ok(())
    }

    #[test]
    fn test_read_from_file() -> io::Result<()> {
        
        let data = "sample data";
        fs::write(TEST_FILE_PATH, data)?;
        let result = read_from_file(TEST_FILE_PATH)?;
        assert_eq!(result, data);
        fs::remove_file(TEST_FILE_PATH)?;
        Ok(())
    }

    #[test]
    fn test_clear_file() -> io::Result<()> {
        
        let data = "Temporary Data";
        fs::write(TEST_FILE_PATH, data)?;
        clear_file(TEST_FILE_PATH)?;
        let contents =
        fs::read_to_string(TEST_FILE_PATH)?;
        assert!(contents.is_empty());
        fs::remove_file(TEST_FILE_PATH)?;
        Ok(())
    }
}
