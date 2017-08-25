use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError, MapAccess};
use std::fmt;
use super::DcName;
use super::AmazonMetaData;

// Field name constants
const NAME: &'static str = "name";
const METADATA: &'static str = "metadata";
// The eureka API has some awful cruft
const CLASS: &'static str = "@class";
const CLASS_VALUE: &'static str = "com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo";
const DATA_CENTER_INFO: &'static str = "DataCenterInfo";
const FIELDS: &'static [&'static str] = &[CLASS, NAME, METADATA];

#[derive(Debug, PartialEq)]
pub struct DataCenterInfo {
    pub name: DcName,
    pub metadata: Option<AmazonMetaData>
}

impl Serialize for DataCenterInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct(DATA_CENTER_INFO, 2)?;
        // weird netflix field
        s.serialize_field(CLASS, CLASS_VALUE)?;
        s.serialize_field(NAME, &self.name)?;
        s.serialize_field(METADATA, &self.metadata)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for DataCenterInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        enum Field { Name, Metadata, Class };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
                D: Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("Expecting `name` or `metadata` ")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                        E: DeError, {
                        match v {
                            NAME => Ok(Field::Name),
                            METADATA => Ok(Field::Metadata),
                            CLASS => Ok(Field::Class),
                            _ => Err(DeError::unknown_field(v, FIELDS))
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DataCenterInfoVisitor;

        impl<'de> Visitor<'de> for DataCenterInfoVisitor {
            type Value = DataCenterInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct DataCenterInfo")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where
                A: MapAccess<'de> {
                let mut maybe_name = None;
                let mut maybe_metadata = None;
                let mut maybe_class: Option<&str> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if maybe_name.is_some() {
                                return Err(DeError::duplicate_field(NAME));
                            }
                            maybe_name = Some(map.next_value()?);
                        },
                        Field::Metadata => {
                            if maybe_metadata.is_some() {
                                return Err(DeError::duplicate_field(METADATA));
                            }
                            maybe_metadata = Some(map.next_value()?);
                        },
                        Field::Class => {
                            maybe_class = Some(map.next_value()?);
                        }
                    }
                }
                let name = maybe_name.ok_or_else(|| DeError::missing_field(NAME));
                debug!("Found ignored field @class {:?} ?", maybe_class);
                Ok(DataCenterInfo {
                    name: name?,
                    metadata: maybe_metadata
                })
            }
        }
        deserializer.deserialize_struct(DATA_CENTER_INFO, FIELDS, DataCenterInfoVisitor)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use serde_json;
    use super::super::amazonmetadata::test::sample_meta_data;

    #[test]
    fn test_serialize_data_center_info() {
        let dci = DataCenterInfo {
            name: DcName::Amazon,
            metadata: Some(AmazonMetaData {
                ami_launch_index: "001a".to_string(),
                local_hostname: "localhost0".to_string(),
                availability_zone: "US_East1a".to_string(),
                instance_id: "instance1a".to_string(),
                public_ip4: "32.23.21.212".to_string(),
                public_hostname: "foo.coma".to_string(),
                ami_manifest_path: "/dev/nulla".to_string(),
                local_ip4: "127.0.0.12".to_string(),
                hostname: "privatefoo.coma".to_string(),
                ami_id: "ami0023".to_string(),
                instance_type: "c4xlarged".to_string()
            })
        };
        let json = sample_data_center();
        let result = serde_json::to_string(&dci).unwrap();
        assert_eq!(json, result);
    }

    #[test]
    fn test_deserialize_data_center_info() {
        let dci = DataCenterInfo {
            name: DcName::Amazon,
            metadata: Some(AmazonMetaData {
                ami_launch_index: "001a".to_string(),
                local_hostname: "localhost0".to_string(),
                availability_zone: "US_East1a".to_string(),
                instance_id: "instance1a".to_string(),
                public_ip4: "32.23.21.212".to_string(),
                public_hostname: "foo.coma".to_string(),
                ami_manifest_path: "/dev/nulla".to_string(),
                local_ip4: "127.0.0.12".to_string(),
                hostname: "privatefoo.coma".to_string(),
                ami_id: "ami0023".to_string(),
                instance_type: "c4xlarged".to_string()
            })
        };
        let json = sample_data_center();
        println!("json {}", json);
        let result = serde_json::from_str(&json).unwrap();
        assert_eq!(dci, result);
    }

    fn sample_data_center() -> String {
        format!("{{\"@class\":\"com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo\",\"name\":\"Amazon\",\"metadata\":{}}}", sample_meta_data())
    }


}