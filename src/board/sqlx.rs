use std::borrow::Cow;

use sqlx::{
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Database, Decode, Encode, Postgres, Type,
};

use crate::{mark, Board, Mark};

use super::BOARD_SIZE;

impl Type<Postgres> for Board {
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

impl Encode<'_, Postgres> for Board {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        encode_by_ref(self, &mut **buf);
        sqlx::encode::IsNull::No
    }
}

pub(crate) fn encode_by_ref(board: &Board, buffer: &mut impl AsMut<Vec<u8>>) {
    let buffer = buffer.as_mut();
    for row in 0..BOARD_SIZE {
        for column in 0..BOARD_SIZE {
            match board.at(row, column) {
                Some(mark) => crate::mark::sqlx::encode_by_ref(&mark, buffer),
                None => buffer.extend(" ".as_bytes()),
            }
        }
    }
}

impl Decode<'_, Postgres> for Board {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(decode(value.as_str()?)?)
    }
}

pub(crate) fn decode(value: &str) -> Result<Board, mark::ParseError> {
    let mut result = Board::default();

    for row in 0..BOARD_SIZE {
        for column in 0..BOARD_SIZE {
            let index = row * BOARD_SIZE + column;
            let char = value.chars().nth(index).unwrap_or(' ').to_string();
            let maybe_mark: Option<Mark> = match char.as_str() {
                " " => None,
                char => Some(char.parse()?),
            };
            result.0[row][column] = maybe_mark;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use claims::assert_err;
    use std::str;

    use crate::mark::{CIRCLE, CROSS};

    use super::*;

    #[test]
    fn encode_empty_ok() -> Result<()> {
        let board = Board::default();
        let mut buffer = Vec::<u8>::default();

        let _ = encode_by_ref(&board, &mut buffer);

        let string = str::from_utf8(&buffer)?;
        let expected = "         ";
        assert_eq!(expected, string);
        Ok(())
    }

    #[test]
    fn encode_some_ok() -> Result<()> {
        let mut board = Board::default();
        board
            .mark(0, 0, Mark::Circle)
            .mark(1, 1, Mark::Cross)
            .mark(2, 0, Mark::Circle)
            .mark(1, 0, Mark::Cross);
        let mut buffer = Vec::<u8>::default();

        let _ = encode_by_ref(&board, &mut buffer);

        let string = str::from_utf8(&buffer)?;
        let expected = format!("{CIRCLE}  {CROSS}{CROSS} {CIRCLE}  ");
        assert_eq!(expected, string);
        Ok(())
    }

    #[test]
    fn encode_full_ok() -> Result<()> {
        let mut board = Board::default();
        board
            .mark(0, 0, Mark::Circle)
            .mark(1, 1, Mark::Cross)
            .mark(2, 0, Mark::Circle)
            .mark(1, 0, Mark::Cross)
            .mark(1, 2, Mark::Circle)
            .mark(0, 1, Mark::Cross)
            .mark(2, 1, Mark::Circle)
            .mark(2, 2, Mark::Cross)
            .mark(0, 2, Mark::Circle);
        let mut buffer = Vec::<u8>::default();

        let _ = encode_by_ref(&board, &mut buffer);

        let string = str::from_utf8(&buffer)?;
        let expected =
            format!("{CIRCLE}{CROSS}{CIRCLE}{CROSS}{CROSS}{CIRCLE}{CIRCLE}{CIRCLE}{CROSS}");
        assert_eq!(expected, string);
        Ok(())
    }

    #[test]
    fn decode_empty_ok() -> Result<()> {
        let value = "         ";

        let result = decode(value)?;

        let expected = Board::default();
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn decode_some_ok() -> Result<()> {
        let value = format!("{CIRCLE}  {CROSS}{CROSS} {CIRCLE}  ");

        let result = decode(value.as_str())?;

        let mut expected = Board::default();
        expected
            .mark(0, 0, Mark::Circle)
            .mark(1, 1, Mark::Cross)
            .mark(2, 0, Mark::Circle)
            .mark(1, 0, Mark::Cross);
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn decode_full_ok() -> Result<()> {
        let value = format!("{CIRCLE}{CROSS}{CIRCLE}{CROSS}{CROSS}{CIRCLE}{CIRCLE}{CIRCLE}{CROSS}");

        let result = decode(value.as_str())?;

        let mut expected = Board::default();
        expected
            .mark(0, 0, Mark::Circle)
            .mark(1, 1, Mark::Cross)
            .mark(2, 0, Mark::Circle)
            .mark(1, 0, Mark::Cross)
            .mark(1, 2, Mark::Circle)
            .mark(0, 1, Mark::Cross)
            .mark(2, 1, Mark::Circle)
            .mark(2, 2, Mark::Cross)
            .mark(0, 2, Mark::Circle);
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn decode_mars_err() -> Result<()> {
        let value = format!("♂");

        let result = decode(value.as_str());

        assert_err!(result);
        Ok(())
    }
}
