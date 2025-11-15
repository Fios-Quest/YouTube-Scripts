mod newtype;

use newtype::*;

#[derive(Debug)]
struct User {
    // Direct access to these properties is forbidden
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

    fn set_email(&mut self, email: Email) {
        self.email = Some(email)
    }

    fn set_date_of_birth(&mut self, date_of_birth: DateOfBirth) {
        self.date_of_birth = Some(date_of_birth);
    }
}

// --- Usage ---

fn main() -> anyhow::Result<()> {
    let mut yuki = User::new(Username::new("Yuki")?);

    yuki.set_email(Email::new("yuki@example.com")?);
    yuki.set_date_of_birth(DateOfBirth::new("2009-05-01")?);

    dbg!(yuki);

    Ok(())
}
