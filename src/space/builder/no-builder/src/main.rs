use newtypes::*;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct UserNotOldEnough;

impl fmt::Display for UserNotOldEnough {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User is not old enough")
    }
}

impl Error for UserNotOldEnough {}

#[derive(Debug)]
struct User {
    username: Username,
    email: EmailAddress,
    date_of_birth: DateOfBirth,
}

impl User {
    fn new(
        username: Username,
        email: EmailAddress,
        date_of_birth: DateOfBirth,
    ) -> Result<Self, UserNotOldEnough> {
        if date_of_birth.get_age() < 21 {
            return Err(UserNotOldEnough);
        }

        Ok(User {
            username,
            email,
            date_of_birth,
        })
    }
}

fn main() -> anyhow::Result<()> {
    let user_instantiate = User {
        username: Username::from_str("Yuki")?,
        email: EmailAddress::from_str("yuki@example.com")?,
        date_of_birth: DateOfBirth::from_str("2009-05-01")?,
    };

    let user_constructor = User::new(
        Username::from_str("Yuki")?,
        EmailAddress::from_str("yuki@example.com")?,
        DateOfBirth::from_str("2009-05-01")?,
    )?;

    dbg!(user_instantiate);
    dbg!(user_constructor);

    Ok(())
}
