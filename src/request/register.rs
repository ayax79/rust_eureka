use super::Instance;
use serde::de::{Deserialize, Deserializer, Error as DeError, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

const REGISTER: &str = "RegisterRequest";
const INSTANCE: &str = "instance";
const FIELDS: &[&str] = &[INSTANCE];

#[derive(Debug, PartialEq)]
pub struct RegisterRequest {
    pub instance: Instance,
}

impl RegisterRequest {
    pub fn new(instance: Instance) -> RegisterRequest {
        RegisterRequest { instance }
    }
}

impl Serialize for RegisterRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct(REGISTER, 1)?;
        s.serialize_field(INSTANCE, &self.instance)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for RegisterRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Instance,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("expecting 'instance'")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: DeError,
                    {
                        match v {
                            INSTANCE => Ok(Field::Instance),
                            _ => Err(DeError::unknown_field(v, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RegisterVisitor;
        impl<'de> Visitor<'de> for RegisterVisitor {
            type Value = RegisterRequest;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expecting struct Register")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut maybe_instance = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Instance => {
                            if maybe_instance.is_some() {
                                return Err(DeError::duplicate_field(INSTANCE));
                            }
                            maybe_instance = Some(map.next_value()?)
                        }
                    }
                }

                let instance = maybe_instance.ok_or_else(|| DeError::missing_field(INSTANCE));
                Ok(RegisterRequest::new(instance?))
            }
        }

        deserializer.deserialize_struct(REGISTER, FIELDS, RegisterVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::super::instance::tests::{build_test_instance, build_test_instance_json};
    use super::*;
    use serde_json;

    #[test]
    fn test_instance_serialization() {
        let json = build_register_json();
        let instance = build_test_instance();
        let rr = RegisterRequest::new(instance);
        let result = serde_json::to_string(&rr).expect("serialization should succeed");

        //                let combined = json.chars().zip(result.chars());
        //                for (a, b) in combined {
        //                    print!("{}", b);
        //                    assert_eq!(a, b);
        //                }
        assert_eq!(json, result);
    }

    #[test]
    fn test_instance_deserialization() {
        let json = build_register_json();
        let instance = build_test_instance();
        let rr = RegisterRequest::new(instance);
        let result = serde_json::from_str(&json).expect("deserialization should succeed");
        assert_eq!(rr, result);
    }

    fn build_register_json() -> String {
        format!("{{\"instance\":{}}}", build_test_instance_json())
    }
}
