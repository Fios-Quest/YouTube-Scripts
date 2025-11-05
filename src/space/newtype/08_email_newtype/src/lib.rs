pub mod email_address {
    use std::{error::Error, fmt};

    #[derive(Debug)]
    pub struct InvalidEmailAddressError;

    impl fmt::Display for InvalidEmailAddressError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Invalid Email Address")
        }
    }

    impl Error for InvalidEmailAddressError {}

    pub struct EmailAddress(String);

    impl fmt::Display for EmailAddress {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl EmailAddress {
        pub fn from_string<S>(email: S) -> Result<EmailAddress, InvalidEmailAddressError>
        where
            S: ToString + AsRef<str>,
        {
            match Self::is_valid(&email) {
                true => Ok(EmailAddress(email.to_string())),
                false => Err(InvalidEmailAddressError),
            }
        }

        pub fn is_valid<S>(email: S) -> bool
        where
            S: AsRef<str>,
        {
            let s = email.as_ref();
            // Must contain an @ that's not the first or last character
            s.contains('@') && !s.starts_with('@') && !s.ends_with('@')
        }
    }
}

#[cfg(test)]
mod tests {
    use super::email_address::*;

    #[test]
    fn test_is_valid() {
        assert!(EmailAddress::is_valid("a@b"));
        assert!(!EmailAddress::is_valid("@ab"));
        assert!(!EmailAddress::is_valid("ab@"));
    }

    #[test]
    fn test_from_string() {
        let valid_email = EmailAddress::from_string("hello@example.com");
        assert!(valid_email.is_ok());

        let invalid_email = EmailAddress::from_string("Ted");
        assert!(invalid_email.is_err());
    }
}
