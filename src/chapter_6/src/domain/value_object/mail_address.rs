use anyhow::Result;

pub struct MailAddress(String);

impl MailAddress {
    pub fn new(mail_address: String) -> Result<Self> {
        Ok(Self(mail_address))
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl sqlx::Type<sqlx::Postgres> for MailAddress {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for MailAddress {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<sqlx::Postgres>>::encode(self.0.clone(), buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for MailAddress {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        <String as sqlx::Decode<sqlx::Postgres>>::decode(value).map(Self)
    }
}
