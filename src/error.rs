use std::fmt;
#[derive(Debug)]
pub struct Error {
    info: String,
}

impl Error {
    pub fn new(info: String) -> Self {
        Self { info: info }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.info)
    }
}
