use newtypes::*;
use std::fmt;

#[derive(Debug)]
pub enum UserStoreError {
    UsernameExists,
    EmailAddressExists,
    UserNotFound,
}

impl fmt::Display for UserStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserStoreError::UsernameExists => write!(f, "Username exists"),
            UserStoreError::EmailAddressExists => write!(f, "Email Address exists"),
            UserStoreError::UserNotFound => write!(f, "User not found"),
        }
    }
}

impl std::error::Error for UserStoreError {}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub username: Username,
    pub email_address: EmailAddress,
}

pub trait UserStore {
    fn store(&self, user: &User) -> anyhow::Result<()>;

    fn get_by_email(&self, email: &EmailAddress) -> anyhow::Result<User>;

    fn get_by_username(&self, username: &Username) -> anyhow::Result<User>;
}
