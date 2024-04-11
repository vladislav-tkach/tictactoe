use crate::mark::ParseMarkError;

use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Database, Decode, Encode, Postgres, Type,
};
use std::{borrow::Cow, fmt::Display, ops::DerefMut, str::FromStr};

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum Mark {
    #[default]
    Circle,
    Cross,
}

pub(crate) const CIRCLE: &str = "○";
pub(crate) const CROSS: &str = "☓";

impl FromStr for Mark {
    type Err = ParseMarkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            CIRCLE => Ok(Mark::Circle),
            CROSS => Ok(Mark::Cross),
            _ => Err(ParseMarkError),
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

impl Type<Postgres> for Mark {
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

impl Encode<'_, Postgres> for Mark {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        encode_by_ref(self, buf.deref_mut());
        sqlx::encode::IsNull::No
    }
}

pub(crate) fn encode_by_ref(mark: &Mark, buffer: &mut impl AsMut<Vec<u8>>) {
    let buffer = buffer.as_mut();
    match mark {
        Mark::Circle => buffer.extend(CIRCLE.as_bytes()),
        Mark::Cross => buffer.extend(CROSS.as_bytes()),
    }
}

impl Decode<'_, Postgres> for Mark {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(value.as_str()?.parse()?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use anyhow::Result;
    use claims::assert_err;
    use std::str;

    #[test]
    fn from_str_circle_parsed_ok() -> Result<()> {
        let result: Mark = CIRCLE.parse()?;

        assert_eq!(Mark::Circle, result);
        Ok(())
    }

    #[test]
    fn from_str_cross_parsed_ok() -> Result<()> {
        let result: Mark = CROSS.parse()?;

        assert_eq!(Mark::Cross, result);
        Ok(())
    }

    #[test]
    fn from_str_mars_parsed_err() -> Result<()> {
        let string = "♂";

        let result = string.parse::<Mark>();

        assert_err!(result);
        Ok(())
    }

    #[test]
    fn encode_circle_ok() -> Result<()> {
        let mark = Mark::Circle;
        let mut buffer = Vec::<u8>::default();

        let _ = encode_by_ref(&mark, &mut buffer);

        let string = str::from_utf8(&buffer)?;
        assert_eq!(CIRCLE, string);
        Ok(())
    }

    #[test]
    fn encode_cross_ok() -> Result<()> {
        let mark = Mark::Cross;
        let mut buffer = Vec::<u8>::default();

        let _ = encode_by_ref(&mark, &mut buffer);

        let string = str::from_utf8(&buffer)?;
        assert_eq!(CROSS, string);
        Ok(())
    }
}