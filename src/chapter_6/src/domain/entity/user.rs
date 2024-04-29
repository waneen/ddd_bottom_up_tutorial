use uuid::Uuid;

use crate::domain::{value_object::MailAddress, UserId, UserName};

pub struct User{
    pub id:UserId,
    pub name :UserName,
    pub mail_address:MailAddress,
}

impl User{
    pub fn new(name:UserName,mail_address:MailAddress)->Self{
        Self{
            id:UserId::new(Uuid::new_v4()),
            name,
            mail_address
        }
    }

    pub fn new_with_id(id:UserId,name:UserName,mail_address:MailAddress)->Self{
        Self{
            id,
            name,
            mail_address,
        }
    }

    pub fn change_name(&mut self,name:UserName)->(){
        self.name=name;
    }

    pub fn change_mail_address(&mut self,mail_address:MailAddress)->(){
        self.mail_address=mail_address;
    }
}

impl std::fmt::Display for User{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.name)
    }
}

pub struct UserUpdateCommand{
    pub id:UserId,
    pub name:Option<UserName>,
    pub mali_address:Option<MailAddress>,
}

