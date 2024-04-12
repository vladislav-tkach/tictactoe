use crate::{mark::ParseError, State};

use sqlx::{
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Database, Decode, Encode, Postgres, Type,
};
use std::{borrow::Cow, ops::DerefMut};

impl Type<Postgres> for State {
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

impl Encode<'_, Postgres> for State {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        encode_by_ref(self, buf.deref_mut());
        sqlx::encode::IsNull::No
    }
}

fn encode_by_ref(state: &State, buffer: &mut impl AsMut<Vec<u8>>) {
    let buffer = buffer.as_mut();
    match state {
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

impl Decode<'_, Postgres> for State {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(decode(&value.as_str()?)?)
    }
}

fn decode(value: &str) -> Result<State, ParseError> {
    let mut chars = value.chars();
    let state = match chars.next().unwrap_or(' ') {
        'P' => {
            let active_role = chars.as_str().parse()?;
            State::Playing(active_role)
        }
        'W' => {
            let winner = chars.as_str().parse()?;
            State::Win(winner)
        }
        'T' => State::Tie,
        _ => return Err(ParseError),
    };

    Ok(state)
}
