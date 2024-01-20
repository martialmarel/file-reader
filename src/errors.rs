pub enum Errors {
    NotArgumentFilepathProvided(String),
    FileNotFound(String),
    ErrorOpeningFile(String),
    ErrorReadingLine(String),
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Errors::NotArgumentFilepathProvided(s) => write!(f, "{}", s),
            Errors::FileNotFound(s) => write!(f, "{}", s),
            Errors::ErrorOpeningFile(s) => write!(f, "{}", s),
            Errors::ErrorReadingLine(s) => write!(f, "{}", s),
        }
    }
}
