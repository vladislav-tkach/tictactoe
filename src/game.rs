use crate::{board::Board, mark::Mark};

#[derive(Default)]
pub struct Game {
    board: Board,
    active_role: Mark,
}

impl Game {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn make_turn(&mut self, row: usize, column: usize) -> Result<State, String> {
        if self.board.at(row, column).is_some() {
            return Err(format!("Tile {row}-{column} is already occupied"));
        }

        self.board.mark(row, column, self.active_role);

        if Self::is_win(&self.board, row, column) {
            return Ok(State::Win(self.active_role));
        }

        if self.board.full() {
            return Ok(State::Tie);
        }

        self.active_role = match self.active_role {
            Mark::Circle => Mark::Cross,
            Mark::Cross => Mark::Circle,
        };

        Ok(State::Playing)
    }

    fn is_win(board: &Board, row: usize, column: usize) -> bool {
        let row_win = board.at(row, 0) == board.at(row, 1) && board.at(row, 1) == board.at(row, 2);
        let column_win = board.at(0, column) == board.at(1, column)
            && board.at(1, column) == board.at(2, column);
        let diagonal_win = (board.at(0, 0) == board.at(1, 1) && board.at(1, 1) == board.at(2, 2))
            && (board.at(2, 0) == board.at(1, 1) && board.at(1, 1) == board.at(0, 2));

        row_win || column_win || diagonal_win
    }
}

pub enum State {
    Playing,
    Win(Mark),
    Tie,
}
