use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
/// The standard crate error type
pub struct Error(&'static str);

impl Error {
    /// Create a new error instance
    pub fn new(msg: &'static str) -> Self {
        Self(msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Result type alias
pub type Result<T> = ::core::result::Result<T, Error>;

/// Shorthand for creating an error
#[macro_export]
macro_rules! err_msg {
    ($msg:expr, $($args:tt)+) => {
        $crate::error::Error::new($crate::error::ErrorKind::$type, format!($msg, $($args)+))
    };
    ($msg:expr) => {{
        $crate::error::Error::new($msg)
    }};
}
