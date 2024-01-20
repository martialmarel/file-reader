use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::errors::Errors;

pub struct FileReader {
    filepath: String,
}

impl FileReader {
    pub fn new(filepath: String) -> FileReader {
        FileReader { filepath }
    }

    pub fn read(&self) -> Result<(), Errors> {
        let file = File::open(&self.filepath);
        let file = match file {
            Ok(file) => file,
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(Errors::FileNotFound(format!("File not found: {}", error)))
                }
                _ => {
                    return Err(Errors::ErrorOpeningFile(format!(
                        "Error opening file: {}",
                        error
                    )))
                }
            },
        };

        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(error) => {
                    return Err(Errors::ErrorReadingLine(format!(
                        "Error reading line: {}",
                        error
                    )))
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::Errors;

    use super::FileReader;

    #[test]
    fn test_read() {
        let file_reader = FileReader::new("README.md".to_owned());
        assert!(file_reader.read().is_ok());
    }

    #[test]
    fn test_read_file_not_found() {
        let file_reader = FileReader::new("not_possible_to_exist_AEBC5678.txt".to_owned());
        let result = file_reader.read();
        assert!(result.is_err());

        if let Err(error) = result {
            match error {
                Errors::FileNotFound(_) => assert!(true),
                _ => assert!(false, "Expected FileNotFound error"),
            }
        } else {
            unreachable!("Expected error");
        }
    }

    // How to test this? Create a file with no read permissions?
    // #[test]
    // fn test_read_error_opening_file() {
    // 		let file_reader = FileReader::new("test3.txt".to_owned());
    // 		assert!(file_reader.read().is_err());
    // }

    // How to test this? Create a file with bad encoding/content?
    // #[test]
    // fn test_read_error_reading_line() {
    // 		let file_reader = FileReader::new("test4.txt".to_owned());
    // 		assert!(file_reader.read().is_err());
    // }
}
