use anyhow::Result;

use crate::domain::{MailAddress, User, UserId, UserName};

pub trait UserRepository{
    // 検索クエリを導入してfindやupdateをUserQuery型を受け取るように変更する？
    fn find_by_user_id(&self,user_id:&UserId)->Option<User>;
    fn find_by_user_name(&self,user_name:&UserName)->Option<User>;
    fn find_by_mail_address(&self,mail_address:&MailAddress)->Option<User>;
    fn save(&self,user:User)->Result<()>;
    fn delete(&self,user:User)->Result<()>;
}