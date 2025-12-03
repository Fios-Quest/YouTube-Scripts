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
struct UserNotOldEnough;

impl fmt::Display for UserNotOldEnough {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User is not old enough")
    }
}

impl Error for UserNotOldEnough {}

struct Unset;

trait UsernameMarker {}
trait EmailAddressMarker {}
trait DateOfBirthMarker {}

impl UsernameMarker for Unset {}
impl EmailAddressMarker for Unset {}
impl DateOfBirthMarker for Unset {}

impl UsernameMarker for Username {}
impl EmailAddressMarker for EmailAddress {}
impl DateOfBirthMarker for DateOfBirth {}

#[derive(Debug)]
struct UserBuilder<U: UsernameMarker, E: EmailAddressMarker, D: DateOfBirthMarker> {
    username: U,
    email: E,
    date_of_birth: D,
}

impl UserBuilder<Unset, Unset, Unset> {
    fn new() -> UserBuilder<Unset, Unset, Unset> {
        UserBuilder {
            username: Unset,
            email: Unset,
            date_of_birth: Unset,
        }
    }
}

impl<U: UsernameMarker, E: EmailAddressMarker, D: DateOfBirthMarker> UserBuilder<U, E, D> {
    fn with_username(self, username: Username) -> UserBuilder<Username, E, D> {
        UserBuilder {
            username,
            email: self.email,
            date_of_birth: self.date_of_birth,
        }
    }

    fn with_email(self, email: EmailAddress) -> UserBuilder<U, EmailAddress, D> {
        UserBuilder {
            username: self.username,
            email,
            date_of_birth: self.date_of_birth,
        }
    }

    fn with_date_of_birth(
        self,
        date_of_birth: DateOfBirth,
    ) -> Result<UserBuilder<U, E, DateOfBirth>, UserNotOldEnough> {
        if date_of_birth.get_age() < 21 {
            return Err(UserNotOldEnough);
        }
        Ok(UserBuilder {
            username: self.username,
            email: self.email,
            date_of_birth,
        })
    }
}

impl UserBuilder<Username, EmailAddress, DateOfBirth> {
    fn build(self) -> User {
        User {
            username: self.username,
            email: self.email,
            date_of_birth: self.date_of_birth,
        }
    }
}

fn main() -> anyhow::Result<()> {
    // We can successfully build a User if we have all the required information
    let user = UserBuilder::new()
        .with_username(Username::from_str("Yuki")?)
        .with_email(EmailAddress::from_str("yuki@example.com")?)
        .with_date_of_birth(DateOfBirth::from_str("2000-01-01")?)?
        .build();

    dbg!(user);

    // But if we don't give all the required information we get an error
    // let user = UserBuilder::new()
    //     .with_username(Username::from_str("Fio")?)
    //     .build();
    // dbg!(user);

    Ok(())
}
