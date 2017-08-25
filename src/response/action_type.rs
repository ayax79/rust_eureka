use std::fmt;
use std::convert::From;
use std::str::FromStr;
use std::error::Error;
use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError};

const ADDED: &'static str = "ADDED";
const DELETED: &'static str = "DELETED";
const MODIFIED: &'static str = "MODIFIED";

#[derive(Debug, PartialEq)]
pub enum ActionType {
    Added,
    Deleted,
    Modified
}

impl ActionType {
    fn values() -> Vec<ActionType> {
        use self::ActionType::*;
        vec![Added, Deleted, Modified]
    }
}

#[derive(Debug)]
pub struct InvalidActionTypeError {
    invalid_value: String
}

impl InvalidActionTypeError {
    pub fn new(invalid_nm: &str) -> Self {
        InvalidActionTypeError {
            invalid_value: invalid_nm.to_owned()
        }
    }
}


impl fmt::Display for InvalidActionTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InvalidActionTypeError({})", self.invalid_value)
    }
}

impl Error for InvalidActionTypeError {
    fn description(&self) -> &str {
        "Not a valid ActionType"
    }
}

impl FromStr for ActionType {
    type Err = InvalidActionTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ADDED => Ok(ActionType::Added),
            DELETED => Ok(ActionType::Deleted),
            MODIFIED => Ok(ActionType::Modified),
            _ => Err(InvalidActionTypeError::new(s))
        }
    }
}

impl From<ActionType> for String {
    fn from(s: ActionType) -> Self {
        match s {
            ActionType::Added => ADDED.to_string(),
            ActionType::Deleted => DELETED.to_string(),
            ActionType::Modified => MODIFIED.to_string(),
        }
    }
}

impl<'a> From<&'a ActionType> for String {
    fn from(s: &ActionType) -> Self {
        match s {
            &ActionType::Added => ADDED.to_string(),
            &ActionType::Deleted => DELETED.to_string(),
            &ActionType::Modified => MODIFIED.to_string(),
        }
    }
}

impl Serialize for ActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        serializer.serialize_str(String::from(self).as_ref())
    }
}

impl<'de> Deserialize<'de> for ActionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        struct ActionTypeVisitor;

        impl<'de> Visitor<'de> for ActionTypeVisitor {
            type Value = ActionType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let values = ActionType::values()
                    .iter()
                    .fold(String::new(), |mut acc, v| {
                        acc.push_str(String::from(v).as_ref());
                        acc
                    });

                formatter.write_fmt(format_args!("Expecting {}", values))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                E: DeError {

                ActionType::from_str(v)
                    .map_err(|err| E::custom(format!("{}", err)))
            }
        }

        deserializer.deserialize_str(ActionTypeVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(ActionType::Added, ActionType::from_str(ADDED).unwrap());
        assert_eq!(ActionType::Deleted, ActionType::from_str(DELETED).unwrap());
        assert_eq!(ActionType::Modified, ActionType::from_str(MODIFIED).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_from_str_invalid() {
        ActionType::from_str("sfd2ef").unwrap();
    }

    #[test]
    fn test_to_string() {
        assert_eq!(ADDED.to_owned(), String::from(ActionType::Added));
        assert_eq!(DELETED.to_owned(), String::from(ActionType::Deleted));
        assert_eq!(MODIFIED.to_owned(), String::from(ActionType::Modified));
    }
}

