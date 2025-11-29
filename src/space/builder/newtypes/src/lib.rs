use std::fmt;
use std::str::FromStr;
use std::error::Error;

#[derive(Debug)]
pub struct ImpossibleError;

impl fmt::Display for ImpossibleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This error should be impossible!")
    }
}

impl Error for ImpossibleError {}

#[derive(Debug)]
pub struct Username(String);

impl FromStr for Username {
    type Err = ImpossibleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct EmailAddress(String);

impl FromStr for EmailAddress {
    type Err = ImpossibleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct DateOfBirth(String);

impl DateOfBirth {
    pub fn get_age(&self) -> u8 {
        16
    }
}

impl FromStr for DateOfBirth {
    type Err = ImpossibleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl fmt::Display for DateOfBirth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
