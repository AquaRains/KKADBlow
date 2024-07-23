use std::error;

#[derive(Debug)]
pub struct ApplicationError
{
    message: String,
    location: Option<Location>,
}

#[derive(Debug)]
pub struct Location
{
    file: &'static str,
    line: u32,
}

impl ApplicationError {
    #[allow(dead_code)]
    pub(crate) fn new<C>(cause: C) -> Self
    where
        C: Into<String>,
    {
        Self {
            message: cause.into(),
            location: None,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_at(cause: String, file: &'static str, line: u32) -> Self

    {
        Self {
            message: cause.into(),
            location: Some(Location { file, line }),
        }
    }
}

impl error::Error for ApplicationError {}

impl std::fmt::Display for ApplicationError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self.location {
            None => write!(f, "{}", self.message),
            Some(ref location) => write!(f, "{} at {}:{}", self.message, location.file, location.line)
        }
    }
}