use serde::Serialize;
use sqlx::{
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    types::Json,
    Database, Decode, Encode, Postgres, Type,
};

use crate::Mark;

impl Type<Postgres> for Mark {
    fn type_info() -> PgTypeInfo {
        <Json<Mark> as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &<Postgres as Database>::TypeInfo) -> bool {
        *ty == <Json<Mark> as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for Mark {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        let json = self
            .serialize(serde_json::value::Serializer)
            .expect("failed to serialize mark");
        json.encode_by_ref(buf)
    }
}

impl Decode<'_, Postgres> for Mark {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes = value.as_bytes()?;
        Ok(serde_json::from_slice(bytes)?)
    }
}
