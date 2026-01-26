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

struct MySqlConfig {
    username: MySqlUsername,
    password: MySqlPassword,
    address: MySqlAddress,
    port: MySqlPort,
}

impl MySqlConfig {
    fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            username: MySqlUsername::from_env("MYSQL_USERNAME")?,
            password: MySqlPassword::from_env("MYSQL_PASSWORD")?,
            address: MySqlAddress::from_env("MYSQL_ADDRESS")?,
            port: MySqlPort::from_env("MYSQL_PORT")?,
        })
    }
}
