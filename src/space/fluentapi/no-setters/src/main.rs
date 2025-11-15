mod newtype;
use newtype::*;

#[derive(Debug)]
struct User {
    // This field will remain immutable
    username: Username,

    // We want to be able to change these fields
    pub email: Option<Email>,
    pub date_of_birth: Option<DateOfBirth>,
}

impl User {
    fn new(username: Username) -> Self {
        Self {
            username,
            email: None,
            date_of_birth: None,
        }
    }
}

// --- Usage ---

fn main() -> Result<(), ()> {
    let mut yuki = User::new(Username::new("Yuki")?);

    yuki.email = Some(Email::new("yuki@example.com")?);
    yuki.date_of_birth = Some(DateOfBirth::new("2009-05-01")?);

    dbg!(yuki);
    Ok(())
}
