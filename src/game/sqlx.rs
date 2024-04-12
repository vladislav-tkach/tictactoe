use std::borrow::Cow;

use sqlx::{
    postgres::{PgArgumentBuffer, PgRow, PgTypeInfo, PgValueRef},
    Database, Decode, Encode, FromRow, Postgres, Row, Type,
};

use crate::{mark::ParseError, Game, State};

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
        encode_by_ref(self, &mut **buf);
        sqlx::encode::IsNull::No
    }
}

fn encode_by_ref(game: &Game, buffer: &mut impl AsMut<Vec<u8>>) {
    let buffer = buffer.as_mut();
    crate::board::sqlx::encode_by_ref(&game.board, buffer);
    buffer.extend(";".as_bytes());
    match game.state {
        State::Playing(active_role) => {
            buffer.extend("P".as_bytes());
            crate::mark::sqlx::encode_by_ref(&active_role, buffer);
        }
        State::Win(winner) => {
            buffer.extend("W".as_bytes());
            crate::mark::sqlx::encode_by_ref(&winner, buffer);
        }
        State::Tie => buffer.extend("T".as_bytes()),
    }
}

impl Decode<'_, Postgres> for Game {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(decode(value.as_str()?)?)
    }
}

fn decode(value: &str) -> Result<Game, ParseError> {
    let mut parts = value.split(';');
    let board_string = parts.next().unwrap_or("");
    let board = crate::board::sqlx::decode(board_string)?;
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
        _ => return Err(ParseError),
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
    use anyhow::Result;
    use std::str;

    use crate::mark::{CIRCLE, CROSS};

    use super::*;

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
        game.make_turn(0, 0)?;
        game.make_turn(1, 1)?;
        game.make_turn(2, 0)?;
        game.make_turn(1, 0)?;
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
        game.make_turn(0, 0)?;
        game.make_turn(1, 1)?;
        game.make_turn(2, 0)?;
        game.make_turn(1, 0)?;
        game.make_turn(1, 2)?;
        game.make_turn(0, 1)?;
        game.make_turn(2, 1)?;
        game.make_turn(2, 2)?;
        game.make_turn(0, 2)?;
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
        expected.make_turn(0, 0)?;
        expected.make_turn(1, 1)?;
        expected.make_turn(2, 0)?;
        expected.make_turn(1, 0)?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn decode_full_ok() -> Result<()> {
        let value =
            format!("{CIRCLE}{CROSS}{CIRCLE}{CROSS}{CROSS}{CIRCLE}{CIRCLE}{CIRCLE}{CROSS};T");

        let result = decode(value.as_str())?;

        let mut expected = Game::default();
        expected.make_turn(0, 0)?;
        expected.make_turn(1, 1)?;
        expected.make_turn(2, 0)?;
        expected.make_turn(1, 0)?;
        expected.make_turn(1, 2)?;
        expected.make_turn(0, 1)?;
        expected.make_turn(2, 1)?;
        expected.make_turn(2, 2)?;
        expected.make_turn(0, 2)?;
        assert_eq!(expected, result);
        Ok(())
    }
}
