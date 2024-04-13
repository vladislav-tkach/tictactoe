pub mod parse_error;

pub use parse_error::ParseError;

pub mod sqlx;

use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum Mark {
    #[default]
    Circle,
    Cross,
}

pub(crate) const CIRCLE: &str = "○";
pub(crate) const CROSS: &str = "☓";

impl FromStr for Mark {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            CIRCLE => Ok(Mark::Circle),
            CROSS => Ok(Mark::Cross),
            _ => Err(ParseError),
        }
    }
}

impl Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mark::Circle => write!(f, "{CIRCLE}"),
            Mark::Cross => write!(f, "{CROSS}"),
        }
    }
}
