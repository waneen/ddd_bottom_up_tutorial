#[derive(Debug, Clone, PartialEq)]
pub struct MailAddress(String);

impl MailAddress {
    pub fn new(mail_address: String) -> Result<Self, MailAddressError> {
        Ok(Self(mail_address))
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum MailAddressError {}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("hoge@example.com")]
    fn test(#[case] mail_address: &str) {
        assert_eq!(
            MailAddress::new(mail_address.to_string()),
            Ok(MailAddress(mail_address.to_string()))
        );
    }
}
