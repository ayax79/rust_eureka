use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError, MapAccess};
use std::fmt;
use super::Application;

const APPLICATION_RESPONSE: &'static str = "ApplicationResponse";
const APPLICATION: &'static str = "application";
const FIELDS: &'static [&'static str] = &[APPLICATION];

#[derive(Debug, PartialEq)]
pub struct ApplicationResponse {
    pub application: Application
}

impl<'a> ApplicationResponse {
    pub fn new(application: Application) -> ApplicationResponse {
        ApplicationResponse {
            application: application
        }
    }
}

impl<'a> Serialize for ApplicationResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct(APPLICATION_RESPONSE, 1)?;
        s.serialize_field(APPLICATION, &self.application)?;
        s.end()
    }
}

impl<'de, 'a> Deserialize<'de> for ApplicationResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        enum Field { Application };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
                D: Deserializer<'de> {
                struct FieldVisitor;
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("expecting 'application'")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                        E: DeError, {
                        match v {
                            APPLICATION => Ok(Field::Application),
                            _ => Err(DeError::unknown_field(v, FIELDS))
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ApplicationResponseVisitor;
        impl<'de> Visitor<'de> for ApplicationResponseVisitor {
            type Value = ApplicationResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expecting struct Application")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where
                A: MapAccess<'de>, {
                let mut maybe_instance = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Application => {
                            if maybe_instance.is_some() {
                                return Err(DeError::duplicate_field(APPLICATION));
                            }
                            maybe_instance = Some(map.next_value()?)
                        }
                    }
                }

                let instance = maybe_instance.ok_or_else(|| DeError::missing_field(APPLICATION));
                Ok(ApplicationResponse::new(instance?))
            }
        }

        deserializer.deserialize_struct(APPLICATION_RESPONSE, FIELDS, ApplicationResponseVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;
    use super::super::instance::tests::{build_test_instance, build_test_instance_json};

    #[test]
    fn test_instance_serialization() {
        let json = build_register_json();
        let instance = build_test_instance();
        let application: Application = Application {
            name: "test_app".to_owned(),
            instance: instance
        };
        let ar = ApplicationResponse::new(application);
        let result = serde_json::to_string(&ar).unwrap();

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
        let application: Application = Application {
            name: "test_app".to_owned(),
            instance: instance
        };
        let ar = ApplicationResponse::new(application);
        let result = serde_json::from_str(&json).unwrap();
        assert_eq!(ar, result);
    }

    fn build_register_json() -> String {

        format!("{{\"application\":{{\"instance\":{}}}}}", build_test_instance_json())
    }
}

