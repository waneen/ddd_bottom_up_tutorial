use anyhow::{anyhow, Result};

pub struct UserName(pub String);

impl UserName{
    pub const MIN_LENGTH:usize=3;
    pub const MAX_LENGTH:usize=20;

    pub fn new(value:String)->Result<Self>{
        if value.is_empty() {
            return Err(anyhow!("ユーザー名が空白です。"));
        }
        if value.len()<Self::MIN_LENGTH{
            return Err(anyhow!("ユーザー名は{}文字以上です",Self::MIN_LENGTH));
        }
        if value.len()>Self::MAX_LENGTH{
            return Err(anyhow!("ユーザー名は{}文字以下です",Self::MAX_LENGTH));
        }
        Ok(Self(value))
    }
}

impl std::fmt::Display for UserName{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}