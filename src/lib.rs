mod command_line;
pub mod errors;

use errors::Errors;

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

    Ok(())
}
