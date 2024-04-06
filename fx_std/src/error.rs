use thiserror::Error;

#[derive(Error, Debug)]
pub enum IOError {
    #[error("IoError: {0}")]
    Error(String),

    #[error("File ({filename:?}) not found ")]
    FileNotFound { filename: String },

    #[error("Error Writing to file")]
    WriteError,

    #[error("Error reading from file")]
    ReadError,
}

impl From<std::io::Error> for IOError {
    fn from(value: std::io::Error) -> Self {
        let error = value.to_string();
        Self::Error(error)
    }
}
