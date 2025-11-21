use std::str::FromStr;

mod newtype;
use newtype::*;

#[derive(Debug)]
struct User {
    username: Username,
    email: Option<Email>,
    date_of_birth: Option<DateOfBirth>,
}

impl User {
    fn new(username: Username) -> Self {
        Self {
            username,
            email: None,
            date_of_birth: None,
        }
    }

    fn set_email(&mut self, email: Email) -> &mut Self {
        self.email = Some(email);
        self
    }

    fn set_date_of_birth(&mut self, date_of_birth: DateOfBirth) -> &mut Self {
        self.date_of_birth = Some(date_of_birth);
        self
    }
}

// --- Usage ---

fn main() -> anyhow::Result<()> {
    let mut yuki = User::new(Username::from_str("Yuki")?);

    let yuki = yuki.set_email(Email::from_str("yuki@example.com")?);
    let yuki = yuki.set_date_of_birth(DateOfBirth::from_str("2009-05-01")?);

    dbg!(yuki);

    Ok(())
}
