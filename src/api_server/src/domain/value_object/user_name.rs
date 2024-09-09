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

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum UserNameError {
    #[error("ユーザー名が空白です。")]
    EmptyUserName,
    #[error("ユーザー名は{min_length}文字以上で入力してください。")]
    TooShort { min_length: usize },
    #[error("ユーザー名は{max_length}文字以上で入力してください。")]
    TooLong { max_length: usize },
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("valid_name", Ok(UserName ("valid_name".to_string() )))]
    #[case("ab", Err(UserNameError::TooShort { min_length: UserName::MIN_LENGTH}))]
    #[case("abcdefghijklmopqrstuvwxyz", Err(UserNameError::TooLong { max_length: UserName::MAX_LENGTH }))]
    fn test(#[case] name: &str, #[case] expected: Result<UserName, UserNameError>) {
        assert_eq!(UserName::new(name.to_string()), expected);
    }
}
