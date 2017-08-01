use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError, MapAccess};
use std::fmt;
use super::DcName;
use super::AmazonMetaData;

// Field name constants
const NAME: &'static str = "name";
const METADATA: &'static str = "metadata";
const DATA_CENTER_INFO: &'static str = "DataCenterInfo";
const FIELDS: &'static [&'static str] = &[NAME, METADATA];

#[derive(Debug, PartialEq)]
pub struct DataCenterInfo {
    pub name: DcName,
    pub metadata: AmazonMetaData
}

impl Serialize for DataCenterInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct(DATA_CENTER_INFO, 2)?;
        s.serialize_field(NAME, &self.name)?;
        s.serialize_field(METADATA, &self.metadata)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for DataCenterInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        enum Field { Name, Metadata };

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
                        }
                    }
                }
                let name = maybe_name.ok_or_else(|| DeError::missing_field(NAME));
                let metadata = maybe_metadata.ok_or_else(|| DeError::missing_field(METADATA));
                Ok(DataCenterInfo {
                    name: name?,
                    metadata: metadata?

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
            metadata: AmazonMetaData {
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
            }
        };
        let json = sample_data_center();
        let result = serde_json::to_string(&dci).unwrap();
        assert_eq!(json, result);
    }

    #[test]
    fn test_deserialize_data_center_info() {
        let dci = DataCenterInfo {
            name: DcName::Amazon,
            metadata: AmazonMetaData {
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
            }
        };
        let json = sample_data_center();
        let result = serde_json::from_str(&json).unwrap();
        assert_eq!(dci, result);
    }

    fn sample_data_center() -> String {
        format!("{{\"name\":\"Amazon\",\"metadata\":{}}}", sample_meta_data())
    }


}