use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError, MapAccess};
use std::iter::Iterator;
use std::fmt;
use std::ops::Add;
use std::convert::From;
use std::str::FromStr;

const LEASE_INFO: &'static str = "LeaseInfo";
const EVICTION_DURATION_IN_SECS: &'static str = "evictionDurationInSecs";
const FIELDS: &'static [&'static str] = &[EVICTION_DURATION_IN_SECS];

#[derive(Debug, PartialEq)]
pub struct LeaseInfo {
    pub eviction_duration_in_secs: Option<u32>
}

impl Serialize for LeaseInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct(LEASE_INFO, 1)?;
        // if not specified we will serialize the default of 90
        let result = self.eviction_duration_in_secs.unwrap_or(90);
        s.serialize_field(EVICTION_DURATION_IN_SECS, &result)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for LeaseInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        enum Field { EvictionDurationInSecs };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
                D: Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("Expecting eviction_duration_in_secs")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                        E: DeError {

                        match v {
                            EVICTION_DURATION_IN_SECS => Ok(Field::EvictionDurationInSecs),
                            _ => Err(DeError::unknown_field(v, FIELDS))
                        }
                    }

                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct LeaseInfoVisitor;

        impl<'de> Visitor<'de> for LeaseInfoVisitor {
            type Value = LeaseInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct LeaseInfoVisitor")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where
                A: MapAccess<'de> {
                let mut maybe_eviction_duration = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::EvictionDurationInSecs => {
                            if maybe_eviction_duration.is_some() {
                                return Err(DeError::duplicate_field(EVICTION_DURATION_IN_SECS));
                            }
                            maybe_eviction_duration = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LeaseInfo{
                    eviction_duration_in_secs: maybe_eviction_duration
                })
            }
        }

        deserializer.deserialize_struct(LEASE_INFO, FIELDS, LeaseInfoVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn test_lease_info_some() {
        let li = LeaseInfo { eviction_duration_in_secs: Some(9600) };
        let json = r#"{"evictionDurationInSecs":9600}"#;
        let result = serde_json::to_string(&li).unwrap();
        assert_eq!(json, result);
    }

    #[test]
    fn test_lease_info_none() {
        let li = LeaseInfo { eviction_duration_in_secs: None };
        let json = r#"{"evictionDurationInSecs":90}"#;
        let result = serde_json::to_string(&li).unwrap();
        assert_eq!(json, result);
    }

    #[test]
    fn test_deserialize_lease_info_some() {
        let li = LeaseInfo { eviction_duration_in_secs: Some(90) };
        let json = r#"{"evictionDurationInSecs":90}"#;
        let result = serde_json::from_str(&json).unwrap();
        assert_eq!(li, result);
    }

}
