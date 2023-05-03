use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error(&'static str);

impl Error {
    pub fn new(msg: &'static str) -> Self {
        Self(msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

pub type Result<T> = ::core::result::Result<T, Error>;
