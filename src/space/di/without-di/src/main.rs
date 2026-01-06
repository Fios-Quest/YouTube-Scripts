use fake_database::*;
use newtypes::*;

struct User {
    email_address: EmailAddress,
    username: Username,
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

        Ok(Self {            mysql
        })
    }

    fn store(&self, user: User) -> anyhow::Result<()> {
        self.mysql.query(
            "
                INSERT INTO users
                  (email_address, username)
                VALUES
                  (?, ?)
            ",
            &[user.email_address.as_str(), user.username.as_str()]
        )?;
        Ok(())
    }
}


fn main() {
    let _user_store = UserStore::new().unwrap();
}
