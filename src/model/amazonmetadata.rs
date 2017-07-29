use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError, MapAccess};
use std::iter::Iterator;
use std::fmt;
use std::ops::Add;
use std::convert::From;
use std::str::FromStr;

const AMI_LAUNCH_INDEX: &'static str = "ami-launch-index";
const LOCAL_HOSTNAME: &'static str = "local-hostname";
const AVAILABILITY_ZONE: &'static str = "availability-zone";
const INSTANCE_ID: &'static str = "instance-id";
const PUBLIC_IPV4: &'static str = "public-ipv4";
const PUBLIC_HOSTNAME: &'static str = "public-hostname";
const AMI_MANIFEST_PATH: &'static str = "ami-manifest-path";
const LOCAL_IPV4: &'static str = "local-ipv4";
const HOSTNAME: &'static str = "hostname";
const AMI_ID: &'static str = "ami-id";
const INSTANCE_TYPE: &'static str = "instance-type";
const JSON_FIELDS: &'static [&'static str] = &[AMI_LAUNCH_INDEX, LOCAL_HOSTNAME, AVAILABILITY_ZONE, INSTANCE_ID,
    PUBLIC_IPV4, PUBLIC_HOSTNAME, AMI_MANIFEST_PATH, LOCAL_IPV4,
    HOSTNAME, AMI_ID, INSTANCE_TYPE];
const RUST_FIELDS: &'static [&'static str] = &["ami_launch_index", "local_hostname", "availability_zone", "instance_id",
    "public_ip4", "public_hostname", "ami_manifest_path", "local_ip4", "hostname", "ami_id", "instance_type"];
const AMAZON_META_DATA: &'static str = "AmazonMetaData";

#[derive(Debug, PartialEq)]
pub struct AmazonMetaData {
    pub ami_launch_index: String,
    pub local_hostname: String,
    pub availability_zone: String,
    pub instance_id: String,
    pub public_ip4: String,
    pub public_hostname: String,
    pub ami_manifest_path: String,
    pub local_ip4: String,
    pub hostname: String,
    pub ami_id: String,
    pub instance_type: String
}

impl Serialize for AmazonMetaData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct(AMAZON_META_DATA, 11)?;
        s.serialize_field(AMI_LAUNCH_INDEX, &self.ami_launch_index)?;
        s.serialize_field(LOCAL_HOSTNAME, &self.local_hostname)?;
        s.serialize_field(AVAILABILITY_ZONE, &self.availability_zone)?;
        s.serialize_field(INSTANCE_ID, &self.instance_id)?;
        s.serialize_field(PUBLIC_IPV4, &self.public_ip4)?;
        s.serialize_field(PUBLIC_HOSTNAME, &self.public_hostname)?;
        s.serialize_field(AMI_MANIFEST_PATH, &self.ami_manifest_path)?;
        s.serialize_field(LOCAL_IPV4, &self.local_ip4)?;
        s.serialize_field(HOSTNAME, &self.hostname)?;
        s.serialize_field(AMI_ID, &self.ami_id)?;
        s.serialize_field(INSTANCE_TYPE, &self.instance_type)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for AmazonMetaData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        enum Field { AmiLaunchIndex, LocalHostname, AvailabilityZone, InstanceId, PublicIp4, PublicHostname, AmiManifestPath, LocalIp4, Hostname, AmiId, InstanceType };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
                D: Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("A AmazonMetaData field (see schema)")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                        E: DeError {
                        match v {
                            AMI_LAUNCH_INDEX => Ok(Field::AmiLaunchIndex),
                            LOCAL_HOSTNAME => Ok(Field::LocalHostname),
                            AVAILABILITY_ZONE => Ok(Field::AvailabilityZone),
                            INSTANCE_ID => Ok(Field::InstanceId),
                            PUBLIC_IPV4 => Ok(Field::PublicIp4),
                            PUBLIC_HOSTNAME => Ok(Field::PublicHostname),
                            AMI_MANIFEST_PATH => Ok(Field::AmiManifestPath),
                            LOCAL_IPV4 => Ok(Field::LocalIp4),
                            HOSTNAME => Ok(Field::Hostname),
                            AMI_ID => Ok(Field::AmiId),
                            INSTANCE_TYPE => Ok(Field::InstanceType),
                            _ => Err(DeError::unknown_field(v, JSON_FIELDS))
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct AmazonMetaDataVisitor;

        impl<'de> Visitor<'de> for AmazonMetaDataVisitor {
            type Value = AmazonMetaData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct AmazonMetaDataVisitor")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where
                A: MapAccess<'de> {
                let mut maybe_ami_launch_index = None;
                let mut maybe_local_hostname = None;
                let mut maybe_availability_zone = None;
                let mut maybe_instance_id = None;
                let mut maybe_public_ip4 = None;
                let mut maybe_public_hostname = None;
                let mut maybe_ami_manifest_path = None;
                let mut maybe_local_ip4 = None;
                let mut maybe_hostname = None;
                let mut maybe_ami_id = None;
                let mut maybe_instance_type = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::AmiLaunchIndex => {
                            if maybe_ami_launch_index.is_some() {
                                return Err(DeError::duplicate_field(AMI_LAUNCH_INDEX));
                            }
                            maybe_ami_launch_index = Some(map.next_value()?)
                        },
                        Field::LocalHostname => {
                            if maybe_local_hostname.is_some() {
                                return Err(DeError::duplicate_field(LOCAL_HOSTNAME));
                            }
                            maybe_local_hostname = Some(map.next_value()?)
                        },
                        Field::AvailabilityZone => {
                            if maybe_availability_zone.is_some() {
                                return Err(DeError::duplicate_field(AVAILABILITY_ZONE));
                            }
                            maybe_availability_zone= Some(map.next_value()?)
                        },
                        Field::InstanceId => {
                            if maybe_instance_id.is_some() {
                                return Err(DeError::duplicate_field(INSTANCE_ID));
                            }
                            maybe_instance_id= Some(map.next_value()?)
                        },
                        Field::PublicIp4 => {
                            if maybe_public_ip4.is_some() {
                                return Err(DeError::duplicate_field(PUBLIC_IPV4));
                            }
                            maybe_public_ip4= Some(map.next_value()?)
                        },
                        Field::PublicHostname => {
                            if maybe_public_hostname.is_some() {
                                return Err(DeError::duplicate_field(PUBLIC_HOSTNAME));
                            }
                            maybe_public_hostname= Some(map.next_value()?)
                        },
                        Field::AmiManifestPath => {
                            if maybe_ami_manifest_path.is_some() {
                                return Err(DeError::duplicate_field(AMI_MANIFEST_PATH));
                            }
                            maybe_ami_manifest_path= Some(map.next_value()?)
                        },
                        Field::LocalIp4 => {
                            if maybe_local_ip4.is_some() {
                                return Err(DeError::duplicate_field(LOCAL_IPV4));
                            }
                            maybe_local_ip4= Some(map.next_value()?)
                        },
                        Field::Hostname => {
                            if maybe_hostname.is_some() {
                                return Err(DeError::duplicate_field(HOSTNAME));
                            }
                            maybe_hostname= Some(map.next_value()?)
                        },
                        Field::AmiId => {
                            if maybe_ami_id.is_some() {
                                return Err(DeError::duplicate_field(AMI_ID));
                            }
                            maybe_ami_id= Some(map.next_value()?)
                        },
                        Field::InstanceType => {
                            if maybe_instance_type.is_some() {
                                return Err(DeError::duplicate_field(INSTANCE_TYPE));
                            }
                            maybe_instance_type= Some(map.next_value()?)
                        }
                    }
                }

                let ami_launch_index = maybe_ami_launch_index.ok_or_else(|| DeError::missing_field(AMI_LAUNCH_INDEX));
                let local_hostname = maybe_local_hostname.ok_or_else(|| DeError::missing_field(LOCAL_HOSTNAME));
                let availability_zone = maybe_availability_zone.ok_or_else(|| DeError::missing_field(AVAILABILITY_ZONE));
                let instance_id = maybe_instance_id.ok_or_else(|| DeError::missing_field(INSTANCE_ID));
                let public_ip4 = maybe_public_ip4.ok_or_else(|| DeError::missing_field(PUBLIC_IPV4));
                let public_hostname = maybe_public_hostname.ok_or_else(|| DeError::missing_field(PUBLIC_HOSTNAME));
                let ami_manifest_path = maybe_ami_manifest_path.ok_or_else(|| DeError::missing_field(AMI_MANIFEST_PATH));
                let local_ip4 = maybe_local_ip4.ok_or_else(|| DeError::missing_field(LOCAL_IPV4));
                let hostname = maybe_hostname.ok_or_else(|| DeError::missing_field(HOSTNAME));
                let ami_id = maybe_ami_id.ok_or_else(|| DeError::missing_field(AMI_ID));
                let instance_type = maybe_instance_type.ok_or_else(|| DeError::missing_field(INSTANCE_TYPE));

                Ok(AmazonMetaData {
                    ami_launch_index: ami_launch_index?,
                    local_hostname: local_hostname?,
                    availability_zone: availability_zone?,
                    instance_id: instance_id?,
                    public_ip4: public_ip4?,
                    public_hostname: public_hostname?,
                    ami_manifest_path: ami_manifest_path?,
                    local_ip4: local_ip4?,
                    hostname: hostname?,
                    ami_id: ami_id?,
                    instance_type: instance_type?
                })
            }
        }
        deserializer.deserialize_struct(AMAZON_META_DATA, RUST_FIELDS, AmazonMetaDataVisitor)
    }
}

mod test {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_amazon_meta_data() {
        let md = AmazonMetaData {
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
        };
        let json = sample_meta_data();

        let result = serde_json::to_string(&md).unwrap();
        assert_eq!(json, result);
    }

    #[test]
    fn test_deserialize_amazon_meta_data() {
        let md = AmazonMetaData {
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
        };
        let json = sample_meta_data();
        let result = serde_json::from_str(&json).unwrap();
        assert_eq!(md, result);
    }

    fn sample_meta_data() -> String {
        r#"{ "ami-launch-index": "001a",
            "local-hostname": "localhost0",
            "availability-zone": "US_East1a",
            "instance-id": "instance1a",
            "public-ipv4": "32.23.21.212",
            "public-hostname": "foo.coma",
            "ami-manifest-path": "/dev/nulla",
            "local-ipv4": "127.0.0.12",
            "hostname": "privatefoo.coma",
            "ami-id": "ami0023",
            "instance-type": "c4xlarged" }"#
            .to_string()
            .replace(" ", "")
            .replace("\n", "")
    }
}