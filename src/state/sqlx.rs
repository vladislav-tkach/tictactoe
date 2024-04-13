use serde::Serialize;
use sqlx::{
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    types::Json,
    Database, Decode, Encode, Postgres, Type,
};

use crate::State;

impl Type<Postgres> for State {
    fn type_info() -> PgTypeInfo {
        <Json<State> as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &<Postgres as Database>::TypeInfo) -> bool {
        *ty == <Json<State> as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for State {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        let json = self
            .serialize(serde_json::value::Serializer)
            .expect("failed to serialize state");
        json.encode_by_ref(buf)
    }
}

impl Decode<'_, Postgres> for State {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes = value.as_bytes()?;
        Ok(serde_json::from_slice(bytes)?)
    }
}
