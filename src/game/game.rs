use crate::{
    board::Board,
    mark::{Mark, ParseMarkError},
    state::state::State,
};

use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgArgumentBuffer, PgRow, PgTypeInfo, PgValueRef},
    Database, Decode, Encode, FromRow, Postgres, Row, Type,
};
use std::{borrow::Cow, ops::DerefMut};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
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

    pub fn make_turn(&mut self, row: usize, column: usize) -> Result<(), String> {
        let active_role = match self.state {
            State::Playing(active_role) => active_role,
            _ => return Err("game is already finished".to_owned()),
        };

        if self.board.at(row, column).is_some() {
            return Err(format!("Tile {row}-{column} is already occupied"));
        }

        self.board.mark(row, column, active_role);

        if Self::is_win(&self.board, row, column) {
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
            other @ _ => other,
        };

        Ok(())
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

impl Type<Postgres> for Game {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &<Postgres as Database>::TypeInfo) -> bool {
        *ty == <&str as Type<Postgres>>::type_info()
            || *ty == <Cow<'_, str> as Type<Postgres>>::type_info()
            || *ty == <Box<str> as Type<Postgres>>::type_info()
            || *ty == <String as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for Game {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        encode_by_ref(self, buf.deref_mut());
        sqlx::encode::IsNull::No
    }
}

fn encode_by_ref(game: &Game, buffer: &mut impl AsMut<Vec<u8>>) {
    let buffer = buffer.as_mut();
    crate::board::board::encode_by_ref(&game.board, buffer);
    buffer.extend(";".as_bytes());
    match game.state {
        State::Playing(active_role) => {
            buffer.extend("P".as_bytes());
            crate::mark::mark::encode_by_ref(&active_role, buffer);
        }
        State::Win(winner) => {
            buffer.extend("W".as_bytes());
            crate::mark::mark::encode_by_ref(&winner, buffer);
        }
        State::Tie => buffer.extend("T".as_bytes()),
    }
}

impl Decode<'_, Postgres> for Game {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(decode(&value.as_str()?)?)
    }
}

fn decode(value: &str) -> Result<Game, ParseMarkError> {
    let mut parts = value.split(';');
    let board_string = parts.next().unwrap_or("");
    let board = crate::board::board::decode(board_string)?;
    let mut state_chars = parts.next().unwrap_or("").chars();
    let state = match state_chars.next().unwrap_or(' ') {
        'P' => {
            let active_role = state_chars.as_str().parse()?;
            State::Playing(active_role)
        }
        'W' => {
            let winner = state_chars.as_str().parse()?;
            State::Win(winner)
        }
        'T' => State::Tie,
        _ => return Err(ParseMarkError),
    };

    Ok(Game { board, state })
}

impl<'r> FromRow<'r, PgRow> for Game {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        row.try_get("state")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mark::mark::{CIRCLE, CROSS};

    use anyhow::Result;
    use std::str;

    #[test]
    fn encode_empty_ok() -> Result<()> {
        let game = Game::default();
        let mut buffer = Vec::<u8>::default();

        let _ = encode_by_ref(&game, &mut buffer);

        let string = str::from_utf8(&buffer)?;
        let expected = format!("         ;P{CIRCLE}");
        assert_eq!(expected, string);
        Ok(())
    }

    #[test]
    fn encode_some_ok() -> Result<()> {
        let mut game = Game::default();
        game.make_turn(0, 0).unwrap();
        game.make_turn(1, 1).unwrap();
        game.make_turn(2, 0).unwrap();
        game.make_turn(1, 0).unwrap();
        let mut buffer = Vec::<u8>::default();

        let _ = encode_by_ref(&game, &mut buffer);

        let string = str::from_utf8(&buffer)?;
        let expected = format!("{CIRCLE}  {CROSS}{CROSS} {CIRCLE}  ;P{CIRCLE}");
        assert_eq!(expected, string);
        Ok(())
    }

    #[test]
    fn encode_full_ok() -> Result<()> {
        let mut game = Game::default();
        game.make_turn(0, 0).unwrap();
        game.make_turn(1, 1).unwrap();
        game.make_turn(2, 0).unwrap();
        game.make_turn(1, 0).unwrap();
        game.make_turn(1, 2).unwrap();
        game.make_turn(0, 1).unwrap();
        game.make_turn(2, 1).unwrap();
        game.make_turn(2, 2).unwrap();
        game.make_turn(0, 2).unwrap();
        let mut buffer = Vec::<u8>::default();

        let _ = encode_by_ref(&game, &mut buffer);

        let string = str::from_utf8(&buffer)?;
        let expected =
            format!("{CIRCLE}{CROSS}{CIRCLE}{CROSS}{CROSS}{CIRCLE}{CIRCLE}{CIRCLE}{CROSS};T");
        assert_eq!(expected, string);
        Ok(())
    }

    #[test]
    fn decode_empty_ok() -> Result<()> {
        let value = format!("         ;P{CIRCLE}");

        let result = decode(value.as_str())?;

        let expected = Game::default();
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn decode_some_ok() -> Result<()> {
        let value = format!("{CIRCLE}  {CROSS}{CROSS} {CIRCLE}  ;P{CIRCLE}");

        let result = decode(value.as_str())?;

        let mut expected = Game::default();
        expected.make_turn(0, 0).unwrap();
        expected.make_turn(1, 1).unwrap();
        expected.make_turn(2, 0).unwrap();
        expected.make_turn(1, 0).unwrap();
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn decode_full_ok() -> Result<()> {
        let value =
            format!("{CIRCLE}{CROSS}{CIRCLE}{CROSS}{CROSS}{CIRCLE}{CIRCLE}{CIRCLE}{CROSS};T");

        let result = decode(value.as_str())?;

        let mut expected = Game::default();
        expected.make_turn(0, 0).unwrap();
        expected.make_turn(1, 1).unwrap();
        expected.make_turn(2, 0).unwrap();
        expected.make_turn(1, 0).unwrap();
        expected.make_turn(1, 2).unwrap();
        expected.make_turn(0, 1).unwrap();
        expected.make_turn(2, 1).unwrap();
        expected.make_turn(2, 2).unwrap();
        expected.make_turn(0, 2).unwrap();
        assert_eq!(expected, result);
        Ok(())
    }
}
