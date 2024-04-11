use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseMarkError;

impl Display for ParseMarkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "provided string was not `{CIRCLE}` or `{CROSS}`".fmt(f)
    }
}

impl Error for ParseMarkError {
    fn description(&self) -> &str {
        "failed to parse mark"
    }
}
