use serde::Serialize;
use sqlx::{
    postgres::{PgArgumentBuffer, PgRow, PgTypeInfo, PgValueRef},
    types::Json,
    Database, Decode, Encode, FromRow, Postgres, Row, Type,
};
use tracing::info;

use crate::Game;

impl Type<Postgres> for Game {
    fn type_info() -> PgTypeInfo {
        <Json<Game> as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &<Postgres as Database>::TypeInfo) -> bool {
        *ty == <Json<Game> as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for Game {
    #[tracing::instrument(name = "Encoding Game for storing in DB", skip(buf), fields(self))]
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        let json = self
            .serialize(serde_json::value::Serializer)
            .expect("failed to serialize game");
        info!("{json}");
        json.encode_by_ref(buf)
    }
}

impl Decode<'_, Postgres> for Game {
    #[tracing::instrument(name = "Decoding Game from DB format", skip(value))]
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        // postgres or something else prepends `☺` for some reason, funny
        let bytes = &value.as_bytes()?[1..];
        info!("{}", value.as_str().expect("failed to get string"));
        Ok(serde_json::from_slice(bytes)?)
    }
}

impl<'r> FromRow<'r, PgRow> for Game {
    #[tracing::instrument(name = "Extracting Game from DB row", skip(row))]
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let state = row.try_get("state");
        info!("{state:?}");
        state
    }
}
