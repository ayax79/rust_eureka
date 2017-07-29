use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError};
use std::iter::Iterator;
use std::fmt;
use std::convert::From;
use std::str::FromStr;
use std::error::Error;

const MY_OWN: &'static str = "MyOwn";
const AMAZON: &'static str = "Amazon";

#[derive(Debug, PartialEq)]
pub enum DcName {
    MyOwn,
    Amazon
}

impl DcName {
    pub fn values() -> Vec<DcName> {
        vec![DcName::MyOwn, DcName::Amazon]
    }
}

#[derive(Debug)]
pub struct InvalidDcNameError {
    invalid_value: String
}

impl InvalidDcNameError {
    pub fn new(invalid_nm: &str) -> Self {
        InvalidDcNameError {
            invalid_value: invalid_nm.to_owned()
        }
    }
}


impl fmt::Display for InvalidDcNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InvalidDcNameError({})", self.invalid_value)
    }
}

impl Error for InvalidDcNameError {
    fn description(&self) -> &str {
        "Not a valid DCName"
    }
}

impl FromStr for DcName {
    type Err = InvalidDcNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MyOwn" => Ok(DcName::MyOwn),
            "Amazon" => Ok(DcName::Amazon),
            _ => Err(InvalidDcNameError::new(s))
        }
    }
}

impl From<DcName> for String {
    fn from(s: DcName) -> Self {
        match s {
            DcName::MyOwn => MY_OWN.to_owned(),
            DcName::Amazon => AMAZON.to_owned()
        }
    }
}

impl<'a> From<&'a DcName> for String {
    fn from(s: &'a DcName) -> Self {
        match s {
            &DcName::MyOwn => MY_OWN.to_owned(),
            &DcName::Amazon => AMAZON.to_owned()
        }
    }
}

impl Serialize for DcName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        serializer.serialize_str(String::from(self).as_ref())
    }
}

impl<'de> Deserialize<'de> for DcName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        struct DcNameVisitor;

        impl<'de> Visitor<'de> for DcNameVisitor {
            type Value = DcName;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let values = DcName::values()
                    .iter()
                    .fold(String::new(), |mut acc, v| {
                        acc.push_str(String::from(v).as_ref());
                        acc
                    });

                formatter.write_fmt(format_args!("Expecting {}", values))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                E: DeError {

                DcName::from_str(v)
                    .map_err(|err| E::custom(format!("{}", err)))
            }
        }

        deserializer.deserialize_str(DcNameVisitor)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(DcName::Amazon, DcName::from_str(AMAZON).unwrap());
        assert_eq!(DcName::MyOwn, DcName::from_str(MY_OWN).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_from_str_invalid() {
        DcName::from_str("sfd2ef").unwrap();
    }

    #[test]
    fn test_to_string() {
        assert_eq!(AMAZON.to_owned(), String::from(DcName::Amazon));
        assert_eq!(MY_OWN.to_owned(), String::from(DcName::MyOwn));
    }
}

