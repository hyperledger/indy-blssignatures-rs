use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
/// The standard crate error type
pub struct Error(Cow<'static, str>);

impl Error {
    /// Create a new error instance
    pub fn new(msg: impl Into<Cow<'static, str>>) -> Self {
        Self(msg.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Result type alias
pub type Result<T> = ::core::result::Result<T, Error>;

/// Shorthand for creating an error
#[macro_export]
macro_rules! err_msg {
    ($msg:expr) => {{
        $crate::error::Error::new($msg)
    }};
    ($msg:expr $(, $arg:expr )+) => {{
        $crate::error::Error::new(format!($msg, $($arg)+))
    }};
}
