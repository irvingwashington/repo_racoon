use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    details: String,
}

impl AppError {
    pub fn new(msg: &str) -> Self {
        AppError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<github_rs::errors::Error> for AppError {
    fn from(error: github_rs::errors::Error) -> Self {
        Self::new(error.description())
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(error: base64::DecodeError) -> Self {
        Self::new(error.description())
    }
}

impl From<std::str::Utf8Error> for AppError {
    fn from(error: std::str::Utf8Error) -> Self {
        Self::new(error.description())
    }
}
