use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct BufferParseError {
    pub description: String
}

impl Error for BufferParseError {}

impl Display for BufferParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.description)?;
        Ok(())
    }
}

impl From<&str> for BufferParseError {
    fn from(value: &str) -> Self {
        Self {
            description: value.to_string()
        }
    }
}