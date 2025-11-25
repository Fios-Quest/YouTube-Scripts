pub fn is_valid_email_address<S>(email: S) -> bool
where
    S: AsRef<str>,
{
    let s = email.as_ref();
    // Must contain an @ that's not the first or last character
    s.contains('@') && !s.starts_with('@') && !s.ends_with('@')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_email_address() {
        assert!(is_valid_email_address("a@b"));
        assert!(!is_valid_email_address("@ab"));
        assert!(!is_valid_email_address("ab@"));
    }
}
