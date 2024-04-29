use anyhow::Result;

pub struct MailAddress(pub String);

impl MailAddress{
    pub fn new(mail_address:String)->Result<Self>{
      Ok(Self(mail_address))
    }
}

