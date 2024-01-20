use std::env;

pub struct CommandLine {
    pub args: Vec<String>,
}

impl CommandLine {
    pub fn parse() -> CommandLine {
        CommandLine {
            args: env::args().collect(),
        }
    }

    fn get_arg(&self, index: usize) -> String {
        self.args[index].clone()
    }

    /// Returns the path of the executable.
    pub fn get_executable_path(&self) -> String {
        self.get_arg(0)
    }

    pub fn get_filepath_to_read(&self) -> Option<String> {
        if self.args.len() == 1 {
            return None;
        }

        Some(self.get_arg(1))
    }
}

#[cfg(test)]
mod tests {
    use super::CommandLine;

    #[test]
    fn test_get_executable_path() {
        let command_line = CommandLine {
            args: vec!["target/debug/file_reader".to_owned(), "test.txt".to_owned()],
        };

        assert_eq!(
            command_line.get_executable_path(),
            "target/debug/file_reader"
        );
    }

    #[test]
    fn test_get_filepath_to_read_whith_arg_provided() {
        let command_line = CommandLine {
            args: vec!["target/debug/file_reader".to_owned(), "test.txt".to_owned()],
        };

        assert!(command_line.get_filepath_to_read().is_some());
        assert_eq!(command_line.get_filepath_to_read().unwrap(), "test.txt");
    }

    #[test]
    fn test_get_filepath_to_read_whithout_arg_provided() {
        let command_line = CommandLine {
            args: vec!["target/debug/file_reader".to_owned()],
        };

        assert!(command_line.get_filepath_to_read().is_none());
    }
}
