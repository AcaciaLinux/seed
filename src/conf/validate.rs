
use std::error::Error;
use std::fmt;

pub trait Validate {
    fn validate(&mut self) -> Result<(), ValidationError>;
}

#[derive(Debug)]
pub struct ValidationError {
    context: String,
    msg: String,
}
impl ValidationError {
    pub fn new(context: &str, message: &str) -> ValidationError {
        ValidationError {
            context: context.to_owned(),
            msg: message.to_owned(),
        }
    }
}
impl Error for ValidationError {}
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed validation: {}: {}", self.context, self.msg)
    }
}
