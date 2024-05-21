use anyhow::Result;
use uuid::Uuid;

pub struct UserId(Uuid);

impl UserId {
    pub fn new(uuid: Uuid) -> Result<Self> {
        Ok(Self(uuid))
    }

    pub fn get(&self) -> Uuid {
        self.0
    }
}

impl sqlx::Type<sqlx::Postgres> for UserId {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for UserId {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        <Uuid as sqlx::Encode<sqlx::Postgres>>::encode(self.0, buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for UserId {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        <Uuid as sqlx::Decode<sqlx::Postgres>>::decode(value).map(Self)
    }
}
