#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AmazonMetaData {
    pub ami_launch_index: String,
    pub local_hostname: String,
    pub availability_zone: String,
    pub instance_id: String,
    pub public_ipv4: String,
    pub public_hostname: String,
    pub ami_manifest_path: String,
    pub local_ipv4: String,
    pub hostname: String,
    pub ami_id: String,
    pub instance_type: String
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_amazon_meta_data() {
        let md = AmazonMetaData {
            ami_launch_index: "001a".to_string(),
            local_hostname: "localhost0".to_string(),
            availability_zone: "US_East1a".to_string(),
            instance_id: "instance1a".to_string(),
            public_ipv4: "32.23.21.212".to_string(),
            public_hostname: "foo.coma".to_string(),
            ami_manifest_path: "/dev/nulla".to_string(),
            local_ipv4: "127.0.0.12".to_string(),
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
            public_ipv4: "32.23.21.212".to_string(),
            public_hostname: "foo.coma".to_string(),
            ami_manifest_path: "/dev/nulla".to_string(),
            local_ipv4: "127.0.0.12".to_string(),
            hostname: "privatefoo.coma".to_string(),
            ami_id: "ami0023".to_string(),
            instance_type: "c4xlarged".to_string()
        };
        let json = sample_meta_data();
        let result = serde_json::from_str(&json).unwrap();
        assert_eq!(md, result);
    }

    pub fn sample_meta_data() -> String {
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