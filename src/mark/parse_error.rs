use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "provided string was not `{CIRCLE}` or `{CROSS}`".fmt(f)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "failed to parse mark"
    }
}
