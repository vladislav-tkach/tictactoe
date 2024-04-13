use serde::Serialize;
use sqlx::{
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    types::Json,
    Database, Decode, Encode, Postgres, Type,
};

use crate::Board;

impl Type<Postgres> for Board {
    fn type_info() -> PgTypeInfo {
        <Json<Board> as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &<Postgres as Database>::TypeInfo) -> bool {
        *ty == <Json<Board> as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for Board {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        let json = self
            .serialize(serde_json::value::Serializer)
            .expect("failed to serialize board");
        json.encode_by_ref(buf)
    }
}

impl Decode<'_, Postgres> for Board {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes = value.as_bytes()?;
        Ok(serde_json::from_slice(bytes)?)
    }
}
