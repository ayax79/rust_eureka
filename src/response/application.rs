use super::Instance;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError, MapAccess};
use std::fmt;

const APPLICATION: &'static str = "Application";
const NAME: &'static str = "name";
const INSTANCE: &'static str = "instance";
const FIELDS: &'static [&'static str] = &[NAME, INSTANCE];

#[derive(Debug, PartialEq)]
pub struct Application {
    pub name: String,
    pub instance: Instance
}


impl<'a> Serialize for Application {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct(APPLICATION, 1)?;
        s.serialize_field(NAME, &self.name)?;
        s.serialize_field(INSTANCE, &self.instance)?;
        s.end()
    }
}

impl<'de, 'a> Deserialize<'de> for Application {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        enum Field { Name, Instance };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
                D: Deserializer<'de> {
                struct FieldVisitor;
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("expecting 'name', 'instance'")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                        E: DeError, {
                        match v {
                            INSTANCE => Ok(Field::Instance),
                            NAME => Ok(Field::Name),
                            _ => Err(DeError::unknown_field(v, FIELDS))
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ApplicationVisitor;
        impl<'de> Visitor<'de> for ApplicationVisitor {
            type Value = Application;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expecting struct Application")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where
                A: MapAccess<'de>, {
                let mut maybe_name = None;
                let mut maybe_instance = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if maybe_name.is_some() {
                                return Err(DeError::duplicate_field(NAME));
                            }
                            maybe_name = Some(map.next_value()?)
                        },
                        Field::Instance => {
                            if maybe_instance.is_some() {
                                return Err(DeError::duplicate_field(INSTANCE));
                            }
                            maybe_instance = Some(map.next_value()?)
                        }
                    }
                }

                let name = maybe_name.ok_or_else(|| DeError::missing_field(NAME));
                let instance = maybe_instance.ok_or_else(|| DeError::missing_field(INSTANCE));
                Ok(Application{
                    name: name?,
                    instance: instance?
                })
            }
        }

        deserializer.deserialize_struct(APPLICATION, FIELDS, ApplicationVisitor)
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
        let name = "test_name";
        let app = Application{
            name: name.to_owned(),
            instance: instance
        };
        let result = serde_json::to_string(&app).unwrap();

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
        let name = "test_name";
        let app = Application{
            name: name.to_owned(),
            instance: instance
        };
        let result = serde_json::from_str(&json).unwrap();
        assert_eq!(app, result);
    }

    fn build_register_json() -> String {
        format!("{{\"name\":\"test_name\",\"instance\":{}}}", build_test_instance_json())
    }
}

