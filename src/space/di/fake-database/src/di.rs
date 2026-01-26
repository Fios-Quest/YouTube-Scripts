use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum EnvironmentError {
    MissingParameter(String),
}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingParameter(parameter) => {
                write!(f, "Missing Environment variable {parameter}")
            }
        }
    }
}

impl Error for EnvironmentError {}

fn get_environment_variable<S: AsRef<str>>(name: S) -> anyhow::Result<String> {
    Ok(std::env::var(name.as_ref())
        .map_err(|_| EnvironmentError::MissingParameter(name.as_ref().to_string()))?)
}

#[derive(Clone)]
struct MySqlUsername(String);

impl MySqlUsername {
    fn from_env<S: AsRef<str>>(name: S) -> anyhow::Result<Self> {
        Ok(Self(get_environment_variable(name)?))
    }
}

#[derive(Clone)]
struct MySqlPassword(String);

impl MySqlPassword {
    fn from_env<S: AsRef<str>>(name: S) -> anyhow::Result<Self> {
        Ok(Self(get_environment_variable(name)?))
    }
}

#[derive(Clone)]
struct MySqlAddress(String);

impl MySqlAddress {
    fn from_env<S: AsRef<str>>(name: S) -> anyhow::Result<Self> {
        Ok(Self(get_environment_variable(name)?))
    }
}

#[derive(Clone)]
struct MySqlPort(u16);

impl MySqlPort {
    fn from_env<S: AsRef<str>>(name: S) -> anyhow::Result<Self> {
        Ok(Self(get_environment_variable(name)?.parse()?))
    }
}

// The field types are just some newtypes I wrote to encapsulate validation.
// While we're not covering this pattern today, I couldn't bring myself to make
// them all Strings after previously explaining why newtypes are so awesome.
#[derive(Clone)]
pub struct MySqlConfig {
    address: MySqlAddress,
    port: MySqlPort,
    username: MySqlUsername,
    password: MySqlPassword,
}

impl MySqlConfig {
    pub fn from_environment() -> anyhow::Result<Self> {
        Ok(Self {
            address: MySqlAddress::from_env("MYSQL_ADDRESS")?,
            port: MySqlPort::from_env("MYSQL_PORT")?,
            username: MySqlUsername::from_env("MYSQL_USERNAME")?,
            password: MySqlPassword::from_env("MYSQL_PASSWORD")?,
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MySql {}

impl MySql {
    pub fn connect(connection: MySqlConfig) -> anyhow::Result<Self> {
        let _ = connection.username;
        let _ = connection.password;
        let _ = connection.address;
        let _ = connection.port;
        Ok(MySql {})
    }

    pub fn query<Q, P>(&self, query: Q, parameters: &[P]) -> anyhow::Result<()>
    where
        Q: AsRef<str>,
        P: AsRef<str>,
    {
        let _ = query;
        let _ = parameters;
        Ok(())
    }
}
