use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError};
use std::iter::Iterator;
use std::fmt;
use std::ops::Add;
use std::convert::From;
use std::str::FromStr;

#[derive(Debug)]
pub struct LeaseInfo {
    pub eviction_duration_in_secs: Option<u32>
}

impl Serialize for LeaseInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct("LeaseInfo", 1)?;
        // if not specified we will serialize the default of 90
        let result = self.eviction_duration_in_secs.unwrap_or(90);
        s.serialize_field("evictionDurationInSecs", &result)?;
        s.end()
    }
}
