use std::fmt;

#[derive(Debug)]
pub enum FustError {
    Io(std::io::Error),
    EditorNotFound,
}

impl fmt::Display for FustError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FustError::Io(err) => write!(f, "IO error: {err}"),
            FustError::EditorNotFound => write!(f, "No suitable editor found"),
        }
    }
}

impl std::error::Error for FustError {}

impl From<std::io::Error> for FustError {
    fn from(err: std::io::Error) -> Self {
        FustError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, FustError>;
