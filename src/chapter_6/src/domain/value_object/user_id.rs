use uuid::Uuid;

pub struct UserId(pub Uuid);

impl UserId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}
