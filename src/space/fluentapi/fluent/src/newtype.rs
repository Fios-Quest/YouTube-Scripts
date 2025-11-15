use thiserror::Error;

#[derive(Debug, Error)]
#[error("Fake error for fake validation")]
pub struct FakeError;

#[derive(Debug)]
pub struct Username(String);

impl Username {
    pub fn new<S: ToString>(username: S) -> Result<Self, FakeError> {
        Ok(Self(username.to_string()))
    }
}

#[derive(Debug)]
pub struct Email(String);

impl Email {
    pub fn new<S: ToString>(email: S) -> Result<Self, FakeError> {
        Ok(Self(email.to_string()))
    }
}

#[derive(Debug)]
pub struct DateOfBirth(String);

impl DateOfBirth {
    pub fn new<S: ToString>(date_of_birth: S) -> Result<Self, FakeError> {
        Ok(Self(date_of_birth.to_string()))
    }
}
