use std::fmt;
// TODO do stuff /td
#[derive(Debug)]
pub struct InvalidPathError {
    pub message: String
}

impl InvalidPathError {
    pub fn new(message: impl Into<String>) -> Self {
       InvalidPathError { message: message.into() } 
    }
}
impl fmt::Display for InvalidPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InvalidPathError {}
