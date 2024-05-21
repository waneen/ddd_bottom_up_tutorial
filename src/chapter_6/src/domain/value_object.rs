mod mail_address;
mod user_id;
mod user_name;

pub use mail_address::MailAddress;
pub use user_id::UserId;
pub use user_name::UserName;

#[macro_export]
macro_rules! define_value_object {
    (pub struct $struct_name:ident($type:ty)) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $struct_name(pub $type);

        impl sqlx::Type<sqlx::Postgres> for $struct_name {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <$type as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }

        impl<'q> sqlx::Encode<'q, sqlx::Postgres> for $struct_name {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> sqlx::encode::IsNull {
                <$type as sqlx::Encode<sqlx::Postgres>>::encode(self.0.clone(), buf)
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for $struct_name {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
                <$type as sqlx::Decode<sqlx::Postgres>>::decode(value).map(Self)
            }
        }
    };
}
