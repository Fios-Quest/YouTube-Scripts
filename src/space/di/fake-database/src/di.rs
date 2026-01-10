use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum EnvironmentError {
    MissingParameter(String),
}

pub struct MySqlConfig {
    username: String,
    password: String,
    address: String,
    port: String,
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

fn get_environment_variable(name: &str) -> anyhow::Result<String> {
    Ok(std::env::var(name).map_err(|_| EnvironmentError::MissingParameter(name.to_string()))?)
}

impl MySqlConfig {
    pub fn from_environment() -> anyhow::Result<Self> {
        Ok(Self {
            username: get_environment_variable("MYSQL_USERNAME")?,
            password: get_environment_variable("MYSQL_PASSWORD")?,
            address: get_environment_variable("MYSQL_ADDRESS")?,
            port: get_environment_variable("MYSQL_PORT")?,
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
