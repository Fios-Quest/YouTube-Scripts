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

    fn set_email(mut self, email: Email) -> Self {
        self.email = Some(email);
        self
    }

    fn set_date_of_birth(mut self, date_of_birth: DateOfBirth) -> Result<Self, TooYoung> {
        if date_of_birth.years_old() > 21 {
            return Err(TooYoung);
        }
        self.date_of_birth = Some(date_of_birth);
        Ok(self)
    }
}

// --- Usage ---

fn main() -> anyhow::Result<()> {
    let yuki = User::new(Username::new("Yuki")?)
        .set_date_of_birth(DateOfBirth::new("2009-05-01")?)?
        .set_email(Email::new("yuki@example.com")?);

    dbg!(yuki);

    Ok(())
}
