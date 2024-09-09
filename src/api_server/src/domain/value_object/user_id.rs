use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(uuid: Uuid) -> Result<Self, UserIdError> {
        Ok(Self(uuid))
    }

    pub fn get(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum UserIdError {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Uuid::new_v4())]
    fn username(#[case] uuid: Uuid) {
        assert_eq!(UserId::new(uuid), Ok(UserId(uuid)));
    }
}
