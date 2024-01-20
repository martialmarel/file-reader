pub enum Errors {
    NotArgumentFilepathProvided(String),
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Errors::NotArgumentFilepathProvided(e) => write!(f, "{}", e),
        }
    }
}
