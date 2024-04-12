pub mod sqlx;

pub mod turn_error;
pub use turn_error::TurnError;

use serde::{Deserialize, Serialize};

use crate::{Board, Mark, State};

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Game {
    board: Board,
    state: State,
}

impl Game {
    pub fn board(&self) -> Board {
        self.board
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn make_turn(&mut self, row: usize, column: usize) -> Result<(), TurnError> {
        let State::Playing(active_role) = self.state else {
            return Err(TurnError::GameFinished);
        };

        if self.board.at(row, column).is_some() {
            return Err(TurnError::CellOccupied);
        }

        self.board.mark(row, column, active_role);

        if self.is_won(row, column) {
            self.state = State::Win(active_role);
            return Ok(());
        }

        if self.board.full() {
            self.state = State::Tie;
            return Ok(());
        }

        self.state = match self.state {
            State::Playing(Mark::Circle) => State::Playing(Mark::Cross),
            State::Playing(Mark::Cross) => State::Playing(Mark::Circle),
            other => other,
        };

        Ok(())
    }

    fn is_won(&self, row: usize, column: usize) -> bool {
        let row_win = self.board.at(row, 0) == self.board.at(row, 1)
            && self.board.at(row, 1) == self.board.at(row, 2);
        let column_win = self.board.at(0, column) == self.board.at(1, column)
            && self.board.at(1, column) == self.board.at(2, column);
        let diagonal_win = (self.board.at(0, 0) == self.board.at(1, 1)
            && self.board.at(1, 1) == self.board.at(2, 2))
            && (self.board.at(2, 0) == self.board.at(1, 1)
                && self.board.at(1, 1) == self.board.at(0, 2));

        row_win || column_win || diagonal_win
    }
}
