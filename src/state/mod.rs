pub mod sqlx;

use serde::{Deserialize, Serialize};

use crate::Mark;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum State {
    Playing(Mark),
    Win(Mark),
    Tie,
}

impl Default for State {
    fn default() -> Self {
        Self::Playing(Mark::default())
    }
}
