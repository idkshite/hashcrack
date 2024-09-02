use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct HashError(pub String);

impl Error for HashError {}

impl fmt::Display for HashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
