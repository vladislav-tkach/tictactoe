use std::fmt::Display;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Mark {
    #[default]
    Circle,
    Cross,
}

impl Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mark::Circle => write!(f, "○"),
            Mark::Cross => write!(f, "☓"),
        }
    }
}
