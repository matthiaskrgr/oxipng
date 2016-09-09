use std::error::Error;
use std::fmt;

#[derive(Debug,Clone)]
pub struct PngError {
    description: String,
}

impl Error for PngError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for PngError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl PngError {
    pub fn new(description: &str) -> PngError {
        PngError { description: description.to_owned() }
    }
}
