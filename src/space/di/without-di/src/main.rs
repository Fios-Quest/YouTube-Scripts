use fake_database::nondi::*;
use newtypes::*;
use std::str::FromStr;

struct User {
    email_address: EmailAddress,
    username: Username,
}

struct Pet {
    name: String,
    butler: User,
}

struct UserStore {
    mysql: MySql,
}

impl UserStore {
    fn new() -> anyhow::Result<Self> {
        let username = std::env::var("MYSQL_USERNAME")?;
        let password = std::env::var("MYSQL_PASSWORD")?;
        let address = std::env::var("MYSQL_ADDRESS")?;
        let port = std::env::var("MYSQL_PORT")?.parse()?;

        let mysql = MySql::connect(address, port, username, password)?;

        Ok(Self { mysql })
    }

    fn store(&self, user: &User) -> anyhow::Result<()> {
        self.mysql.query(
            "
                INSERT INTO users
                  (email_address, username)
                VALUES
                  (?, ?)
            ",
            &[user.email_address.as_str(), user.username.as_str()],
        )
    }
}

struct PetStore {
    mysql: MySql,
}

impl PetStore {
    fn new() -> anyhow::Result<Self> {
        let username = std::env::var("MYSQL_USERNAME")?;
        let password = std::env::var("MYSQL_PASSWORD")?;
        let address = std::env::var("MYSQL_ADDRESS")?;
        let port = std::env::var("MYSQL_PORT")?.parse()?;

        let mysql = MySql::connect(address, port, username, password)?;

        Ok(Self { mysql })
    }

    fn store(&self, pet: &Pet) -> anyhow::Result<()> {
        self.mysql.query(
            "
                INSERT INTO pets
                  (carer, name)
                VALUES
                  (?, ?)
            ",
            &[pet.butler.username.as_str(), pet.name.as_str()],
        )
    }
}

fn main() -> anyhow::Result<()> {
    let pet_store = PetStore::new()?;

    let user_store = UserStore::new()?;

    let daniel = User {
        username: Username::from_str("Daniel")?,
        email_address: EmailAddress::from_str("daniel@example.com")?,
    };

    user_store.store(&daniel)?;

    let yuki = Pet {
        butler: daniel,
        name: String::from("Yuki"),
    };

    pet_store.store(&yuki)?;

    Ok(())
}
