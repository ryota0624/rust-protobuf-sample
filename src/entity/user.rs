use std::collections::HashMap;

use crate::repository::{Entity, Error, Repository};

#[derive(PartialEq, Hash, Eq, Clone, Debug)]
pub struct UserId(pub String);

#[derive(Clone, Debug)]
pub struct User {
    id: UserId,
}

impl Entity for User {
    type Id = UserId;

    fn entity_id(&self) -> &Self::Id {
        self.id()
    }
}

impl User {
    pub fn new(id: UserId) -> User {
        User { id }
    }

    fn id(&self) -> &UserId {
        &self.id
    }

    fn name(&self) -> String {
        todo!()
    }

    fn email(&self) -> String {
        todo!()
    }

    fn icon_url(&self) -> Option<String> {
        todo!()
    }
}

pub trait UserRepository: Repository<User> {}

#[derive(Clone, Debug)]
pub struct UserRepositoryOnMemory {
    user_map: HashMap<UserId, User>,
}

impl Repository<User> for UserRepositoryOnMemory {
    type S = Self;

    fn find_by_id(&self, id: &UserId) -> Option<&User> {
        self.user_map.get(id)
    }

    fn store(mut self, t: User) -> Result<Self, Error<User>> {
        if self.user_map.get(t.id()).is_some() {
            Err(Error::EntityAlreadyExist())
        } else {
            self.user_map.insert(t.id().clone(), t);
            Ok(self)
        }
    }
}

impl UserRepository for UserRepositoryOnMemory {}

impl Default for UserRepositoryOnMemory {
    fn default() -> UserRepositoryOnMemory {
        UserRepositoryOnMemory {
            user_map: HashMap::new(),
        }
    }
}
