use std::fmt::Error;
use std::ops::Deref;
use std::str::FromStr;
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq)]
struct EmailString(String);

impl EmailString {
    fn new(email: &str) -> Result<Self, Error> {
        if Self::is_valid_email(email) {
            Ok(Self(String::from(email)))
        } else {
            Err(Error)
        }
    }

    fn is_valid_email(email: &str) -> bool {
        email.contains('@')
    }

    fn as_str(&self) -> &str {
        &self.0
    }

    fn into_string(self) -> String {
        self.0
    }
}

impl Deref for EmailString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for EmailString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<String> for EmailString {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl TryFrom<&str> for EmailString {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<EmailString> for String {
    fn from(email: EmailString) -> Self {
        email.0
    }
}

struct Random<T> {
    values: [T; 3],
}

impl<T> Random<T> {
    fn new(val1: T, val2: T, val3: T) -> Self {
        Random {
            values: [val1, val2, val3]
        }
    }

    fn get(&self) -> &T {
        let index = rand::thread_rng().gen_range(0..3);
        &self.values[index]
    }

    fn get_mut(&mut self) -> &mut T {
        let index = rand::thread_rng().gen_range(0..3);
        &mut self.values[index]
    }
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_string_valid() {
        let email = EmailString::new("test@gmail.com").unwrap();
        assert_eq!(email.as_str(), "test@gmail.com");
        assert_eq!(*email, "test@gmail.com");
    }

    #[test]
    fn test_email_string_invalid() {
        assert!(EmailString::new("invalidemail").is_err())
    }

    #[test]
    fn test_email_string_conversions() {
        // FromStr
        let email: EmailString = "test@gmail.com".parse().unwrap();
        assert_eq!(email.as_str(), "test@gmail.com");

        // TryFrom<String>
        let email = EmailString::try_from(String::from("test@gmail.com")).unwrap();
        assert_eq!(email.as_str(), "test@gmail.com");

        // TryFrom<&str>
        let email = EmailString::try_from("test@gmail.com").unwrap();
        assert_eq!(email.as_str(), "test@gmail.com");

        // Into<String>
        let email_string: String = email.into();
        assert_eq!(email_string, "test@gmail.com");
    }

    #[test]
    fn test_email_string_deref() {
        let email = EmailString::new("aa@aa.com").unwrap();
        assert_eq!(email.len(), 9)
    }

    #[test]
    fn test_random_basic() {
        let random = Random::new(1, 2, 3);
        for _ in 0..10 {
            let val = *random.get();
            assert!(val == 1 || val == 2 || val == 3)
        }
    }
}
