use newtypes::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct MySqlUsername(String);

impl MySqlUsername {
    fn from_env<S: AsRef<str>>(env_name: S) -> anyhow::Result<Self> {
        let _ = env_name;
        Ok(Self("".to_string()))
    }
}

impl FromStr for MySqlUsername {
    type Err = ImpossibleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug)]
pub struct MySqlPassword(String);

impl MySqlPassword {
    fn from_env<S: AsRef<str>>(env_name: S) -> anyhow::Result<Self> {
        let _ = env_name;
        Ok(Self("".to_string()))
    }
}

impl FromStr for MySqlPassword {
    type Err = ImpossibleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug)]
pub struct MySqlAddress(String);

impl MySqlAddress {
    fn from_env<S: AsRef<str>>(env_name: S) -> anyhow::Result<Self> {
        let _ = env_name;
        Ok(Self("".to_string()))
    }
}

impl FromStr for MySqlAddress {
    type Err = ImpossibleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug)]
pub struct MySqlPort(String);

impl MySqlPort {
    fn from_env<S: AsRef<str>>(env_name: S) -> anyhow::Result<Self> {
        let _ = env_name;
        Ok(Self("".to_string()))
    }
}

impl FromStr for MySqlPort {
    type Err = ImpossibleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

// ----------------------------------------------------------------------------

struct MySqlConfig {
    address: MySqlAddress,
    port: MySqlPort,
    username: MySqlUsername,
    password: MySqlPassword,
}

impl MySqlConfig {
    fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            address: MySqlAddress::from_env("MYSQL_ADDRESS")?,
            port: MySqlPort::from_env("MYSQL_PORT")?,
            username: MySqlUsername::from_env("MYSQL_USERNAME")?,
            password: MySqlPassword::from_env("MYSQL_PASSWORD")?,
        })
    }
}

// ----------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
struct MySql {}

impl MySql {
    fn connect(connection: MySqlConfig) -> anyhow::Result<Self> {
        let _ = connection.username;
        let _ = connection.password;
        let _ = connection.address;
        let _ = connection.port;
        Ok(MySql {})
    }

    fn query<Q, P>(&self, query: Q, parameters: &[P]) -> anyhow::Result<()>
    where
        Q: AsRef<str>,
        P: AsRef<str>,
    {
        let _ = query;
        let _ = parameters;
        Ok(())
    }
}

// ----------------------------------------------------------------------------

struct User {
    email_address: EmailAddress,
    username: Username,
}

struct Pet {
    butler: User,
    name: String,
}

struct UserStore {
    mysql: MySql,
}

impl UserStore {
    fn new(mysql: MySql) -> Self {
        Self { mysql }
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

    fn get_by_email(&self, email: &EmailAddress) -> anyhow::Result<User> {
        let _ = email;
        Ok(User {
            email_address: EmailAddress::from_str("")?,
            username: Username::from_str("")?,
        })
    }

    fn get_by_username(&self, username: &Username) -> anyhow::Result<User> {
        let _ = username;
        Ok(User {
            email_address: EmailAddress::from_str("")?,
            username: Username::from_str("")?,
        })
    }
}

struct PetStore {
    mysql: MySql,
}

impl PetStore {
    fn new(mysql: MySql) -> Self {
        Self { mysql }
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
    let mysql_config = MySqlConfig::from_env()?;

    let mysql = MySql::connect(mysql_config)?;

    let user_store = UserStore::new(mysql);
    let pet_store = PetStore::new(mysql);

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

    user_store.get_by_email(&EmailAddress::from_str("")?)?;
    user_store.get_by_username(&Username::from_str("")?)?;

    Ok(())
}
