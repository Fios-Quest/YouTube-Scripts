use std::str::FromStr;
use newtypes::*;

#[derive(Debug)]
struct User {
    username: Username,
    email: EmailAddress,
    date_of_birth: DateOfBirth,
}

impl User {
    fn new(username: Username, email: EmailAddress, date_of_birth: DateOfBirth) -> Result<Self, ()> {
        if date_of_birth.get_age() < 21 {
            return Err(())
        }

        Ok(User {
            username,
            email,
            date_of_birth,
        })
    }
}

fn main() -> anyhow::Result<()> {
    let user = User {
        username: Username::from_str("Yuki")?,
        email: EmailAddress::from_str("yuki@example.com")?,
        date_of_birth: DateOfBirth::from_str("2009-05-01")?,
    };

    let user = User::new(
        Username::from_str("Yuki")?,
        EmailAddress::from_str("yuki@example.com")?,
        DateOfBirth::from_str("2009-05-01")?,
    );
    Ok(())
}
