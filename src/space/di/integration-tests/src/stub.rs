use newtypes::*;
use std::cell::RefCell;

use crate::user_store::{User, UserStore, UserStoreError};

#[derive(Default)]
pub struct StubUserStore {
    users: RefCell<Vec<User>>,
}

impl StubUserStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl UserStore for StubUserStore {
    fn store(&self, user: &User) -> anyhow::Result<()> {
        if self.get_by_email(&user.email_address).is_ok() {
            return Err(UserStoreError::EmailAddressExists.into());
        }
        if self.get_by_username(&user.username).is_ok() {
            return Err(UserStoreError::UsernameExists.into());
        }
        self.users.borrow_mut().push(user.clone());
        Ok(())
    }

    fn get_by_email(&self, email: &EmailAddress) -> anyhow::Result<User> {
        self.users
            .borrow()
            .iter()
            .find(|user| &user.email_address == email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound.into())
    }

    fn get_by_username(&self, username: &newtypes::Username) -> anyhow::Result<User> {
        self.users
            .borrow()
            .iter()
            .find(|user| &user.username == username)
            .cloned()
            .ok_or(UserStoreError::UserNotFound.into())
    }
}
