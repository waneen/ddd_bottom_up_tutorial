#[derive(Debug, PartialEq)]
pub struct UserName(String);

impl UserName {
    pub const MIN_LENGTH: usize = 3;
    pub const MAX_LENGTH: usize = 20;

    pub fn new(value: String) -> Result<Self, UserNameError> {
        if value.is_empty() {
            return Err(UserNameError::EmptyUserName);
        }
        if value.len() < Self::MIN_LENGTH {
            return Err(UserNameError::TooShort {
                min_length: Self::MIN_LENGTH,
            });
        }
        if value.len() > Self::MAX_LENGTH {
            return Err(UserNameError::TooLong {
                max_length: Self::MAX_LENGTH,
            });
        }
        Ok(Self(value))
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::fmt::Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl sqlx::Type<sqlx::Postgres> for UserName {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for UserName {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<sqlx::Postgres>>::encode(self.0.clone(), buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for UserName {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        <String as sqlx::Decode<sqlx::Postgres>>::decode(value).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("valid_name", Ok(UserName ("valid_name".to_string() )))]
    #[case("ab", Err(UserNameError::TooShort { min_length: UserName::MIN_LENGTH}))]
    #[case("abcdefghijklmopqrstuvwxyz", Err(UserNameError::TooLong { max_length: UserName::MAX_LENGTH }))]
    fn test_username_new(#[case] name: &str, #[case] expected: Result<UserName, UserNameError>) {
        assert_eq!(UserName::new(name.to_string()), expected);
    }
}

#[derive(Debug, PartialEq)]
pub enum UserNameError {
    EmptyUserName,
    TooShort { min_length: usize },
    TooLong { max_length: usize },
}

// MEMO: ThisErrorでもいいかもしれない
impl std::fmt::Display for UserNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyUserName => write!(f, "ユーザー名が空白です。"),
            Self::TooShort { min_length } => {
                write!(f, "ユーザー名は{}文字以上で入力してください。", min_length)
            }
            Self::TooLong { max_length } => {
                write!(f, "ユーザー名は{}文字以上で入力してください。", max_length)
            }
        }
    }
}

impl std::error::Error for UserNameError {}
