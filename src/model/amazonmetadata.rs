use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError};
use std::iter::Iterator;
use std::fmt;
use std::ops::Add;
use std::convert::From;
use std::str::FromStr;

#[derive(Debug)]
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
        let mut s = serializer.serialize_struct("AmazonMetaData", 11)?;
        s.serialize_field("ami-launch-index", &self.ami_launch_index)?;
        s.serialize_field("local-hostname", &self.local_hostname)?;
        s.serialize_field("availability-zone", &self.availability_zone)?;
        s.serialize_field("instance-id", &self.instance_id)?;
        s.serialize_field("public-ipv4", &self.public_ip4)?;
        s.serialize_field("public-hostname", &self.public_hostname)?;
        s.serialize_field("ami-manifest-path", &self.ami_manifest_path)?;
        s.serialize_field("local-ipv4", &self.local_ip4)?;
        s.serialize_field("hostname", &self.hostname)?;
        s.serialize_field("ami-id", &self.ami_id)?;
        s.serialize_field("instance-type", &self.instance_type)?;
        s.end()
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