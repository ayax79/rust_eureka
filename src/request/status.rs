use serde::de::{Deserialize, Deserializer, Error as DeError, Visitor};
use serde::ser::{Serialize, Serializer};
use std::convert::From;
use std::fmt;
use std::iter::Iterator;

const UP: &str = "UP";
const DOWN: &str = "DOWN";
const STARTING: &str = "STARTING";
const OUT_OF_SERVICE: &str = "OUT_OF_SERVICE";
const UNKNOWN: &str = "UNKNOWN";

#[derive(Debug, PartialEq)]
pub enum Status {
    Up,
    Down,
    Starting,
    OutOfService,
    Unknown,
}

impl Status {
    fn values() -> Vec<Status> {
        use self::Status::*;
        vec![Up, Down, Starting, OutOfService]
    }
}

impl From<&str> for Status {
    fn from(str: &str) -> Self {
        match str {
            UP => Status::Up,
            DOWN => Status::Down,
            STARTING => Status::Starting,
            OUT_OF_SERVICE => Status::OutOfService,
            _ => Status::Unknown,
        }
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        Status::from(s.as_ref())
    }
}

impl From<Status> for String {
    fn from(s: Status) -> Self {
        match s {
            Status::Up => UP.to_string(),
            Status::Down => DOWN.to_string(),
            Status::Starting => STARTING.to_string(),
            Status::OutOfService => OUT_OF_SERVICE.to_string(),
            _ => UNKNOWN.to_string(),
        }
    }
}

impl From<&Status> for String {
    fn from(s: &Status) -> Self {
        match *s {
            Status::Up => UP.to_string(),
            Status::Down => DOWN.to_string(),
            Status::Starting => STARTING.to_string(),
            Status::OutOfService => OUT_OF_SERVICE.to_string(),
            Status::Unknown => UNKNOWN.to_string(),
        }
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(String::from(self).as_ref())
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatusVisitor;

        impl<'de> Visitor<'de> for StatusVisitor {
            type Value = Status;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let values = Status::values().iter().fold(String::new(), |mut acc, v| {
                    acc.push_str(String::from(v).as_ref());
                    acc
                });

                formatter.write_fmt(format_args!("Expecting {}", values))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(From::from(v))
            }
        }

        deserializer.deserialize_str(StatusVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_string_ref() {
        let up = Status::from(UP);
        assert_eq!(Status::Up, up);
    }

    #[test]
    fn test_from_string() {
        let up = Status::from(UP.to_owned());
        assert_eq!(Status::Up, up);
    }
}
