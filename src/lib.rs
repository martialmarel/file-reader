mod command_line;
pub mod errors;
mod file_reader;

use errors::Errors;
use file_reader::FileReader;

use crate::command_line::CommandLine;

pub fn process() -> Result<(), Errors> {
    let command_line = CommandLine::parse();

    let filepath = match command_line.get_filepath_to_read() {
        Some(filepath) => filepath,
        None => {
            return Err(Errors::NotArgumentFilepathProvided(
                "No argument filepath provided.".to_owned(),
            ))
        }
    };

    let file_reader = FileReader::new(filepath);
    file_reader.read()?;

    Ok(())
}
