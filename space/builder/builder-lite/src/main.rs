use newtypes::*;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct User {
    username: Username,
    email: EmailAddress,
    date_of_birth: DateOfBirth,
}

#[derive(Debug)]
enum UserBuilderError {
    NoUsername,
    NoEmailAddress,
    NoDateOfBirth,
    NotOldEnough,
}

impl fmt::Display for UserBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoUsername => write!(f, "No username provided"),
            Self::NoEmailAddress => write!(f, "No email address provided"),
            Self::NoDateOfBirth => write!(f, "No no date of birth provided"),
            Self::NotOldEnough => write!(f, "User is not old enough"),
        }
    }
}

impl Error for UserBuilderError {}

#[derive(Default)]
struct UserBuilder {
    username: Option<Username>,
    email: Option<EmailAddress>,
    date_of_birth: Option<DateOfBirth>,
}

impl UserBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn with_username(mut self, username: Username) -> Self {
        self.username = Some(username);
        self
    }

    fn with_email(mut self, email: EmailAddress) -> Self {
        self.email = Some(email);
        self
    }

    fn with_date_of_birth(mut self, date_of_birth: DateOfBirth) -> Result<Self, UserBuilderError> {
        if date_of_birth.get_age() < 21 {
            return Err(UserBuilderError::NotOldEnough);
        }
        self.date_of_birth = Some(date_of_birth);
        Ok(self)
    }

    fn build(self) -> Result<User, UserBuilderError> {
        let username = self.username.ok_or(UserBuilderError::NoUsername)?;
        let email = self.email.ok_or(UserBuilderError::NoEmailAddress)?;
        let date_of_birth = self.date_of_birth.ok_or(UserBuilderError::NoDateOfBirth)?;

        Ok(User {
            username,
            email,
            date_of_birth,
        })
    }
}

fn main() -> anyhow::Result<()> {
    // We can successfully build a User if we have all the required information
    let user_result = UserBuilder::new()
        .with_username(Username::from_str("Yuki")?)
        .with_email(EmailAddress::from_str("yuki@example.com")?)
        .with_date_of_birth(DateOfBirth::from_str("2000-01-01")?)?
        .build();
    assert!(user_result.is_ok());

    // But if we don't give all the required information we get an error
    let user_result = UserBuilder::new()
        .with_username(Username::from_str("Fio")?)
        .build();
    assert!(user_result.is_err());

    Ok(())
}
