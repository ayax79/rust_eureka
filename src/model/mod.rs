pub mod status;

use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError};
use std::iter::Iterator;
use std::fmt;
use std::ops::Add;
use std::convert::From;
use std::str::FromStr;
use self::status::Status;

#[derive(Debug)]
pub enum DcName {
    MyOwn,
    Amazon
}

impl Serialize for DcName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let result = match self {
            &DcName::MyOwn => "MyOwn",
            &DcName::Amazon => "Amazon"
        };

        serializer.serialize_str(result)
    }
}

#[derive(Debug)]
pub struct AmazonMetaData {
    ami_launch_index: String,
    local_hostname: String,
    availability_zone: String,
    instance_id: String,
    public_ip4: String,
    public_hostname: String,
    ami_manifest_path: String,
    local_ip4: String,
    hostname: String,
    ami_id: String,
    instance_type: String
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

#[derive(Debug)]
pub struct DataCenterInfo {
    name: DcName,
    metadata: AmazonMetaData
}

impl Serialize for DataCenterInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct("DataCenterInfo", 2)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.end()
    }
}


#[derive(Debug)]
pub struct LeaseInfo {
    eviction_duration_in_secs: Option<u32>
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


#[derive(Debug)]
pub struct Instance {
    host_name: String,
    app: String,
    ip_addr: String,
    vip_address: String,
    secure_vip_address: String,
    status: Status,
    port: Option<u16>,
    secure_port: Option<u16>,
    homepage_url: String,
    status_page_url: String,
    health_check_url: String,
    data_center_info: DataCenterInfo,
    lease_info: Option<LeaseInfo>,
    metadata: Vec<String>
}

impl Serialize for Instance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct("Instance", 14)?;
        s.serialize_field("hostName", &self.host_name)?;
        s.serialize_field("app", &self.app)?;
        s.serialize_field("ipAddr", &self.ip_addr)?;
        s.serialize_field("vipAddress", &self.vip_address)?;
        s.serialize_field("secureVipAddress", &self.secure_vip_address)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("port", &self.port)?;
        s.serialize_field("securePort", &self.secure_port)?;
        s.serialize_field("homePageUrl", &self.homepage_url)?;
        s.serialize_field("statusPageUrl", &self.status_page_url)?;
        s.serialize_field("healthCheckUrl", &self.health_check_url)?;
        s.serialize_field("dataCenterInfo", &self.data_center_info)?;
        s.serialize_field("leaseInfo", &self.lease_info)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_instance() {
        let json = r#"{
           "hostName": "Foo",
           "app": "Bar",
           "ipAddr": "3.128.2.12",
           "vipAddress": "127.0.0.1",
           "secureVipAddress": "127.0.0.2",
           "status": "UP",
           "port": 80,
           "securePort": 443,
           "homePageUrl": "http://google.com",
           "statusPageUrl": "http://nytimes.com",
           "healthCheckUrl": "http://washingtonpost.com",
           "dataCenterInfo": {"name":"Amazon","metadata":
           {
                "ami-launch-index": "001a",
                "local-hostname": "localhost0",
                "availability-zone": "US_East1a",
                "instance-id": "instance1a",
                "public-ipv4": "32.23.21.212",
                "public-hostname": "foo.coma",
                "ami-manifest-path": "/dev/nulla",
                "local-ipv4": "127.0.0.12",
                "hostname": "privatefoo.coma",
                "ami-id": "ami0023",
                "instance-type": "c4xlarged"
           }},
           "leaseInfo": {"evictionDurationInSecs":9600},
           "metadata": ["something"]
        }"#
            .to_string()
            .replace(" ", "")
            .replace("\n", "");

        let instance = Instance {
            host_name: "Foo".to_string(),
            app: "Bar".to_string(),
            ip_addr: "3.128.2.12".to_string(),
            vip_address: "127.0.0.1".to_string(),
            secure_vip_address: "127.0.0.2".to_string(),
            status: Status::Up,
            port: Some(80),
            secure_port: Some(443),
            homepage_url: "http://google.com".to_string(),
            status_page_url: "http://nytimes.com".to_string(),
            health_check_url: "http://washingtonpost.com".to_string(),
            data_center_info: DataCenterInfo {
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
            },
            lease_info: Some(LeaseInfo {
                eviction_duration_in_secs: Some(9600)
            }),
            metadata: vec!["something".to_string()]
        };

        let result = serde_json::to_string(&instance).unwrap();
        assert_eq!(json, result);
    }


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

    fn sample_data_center() -> String {
        format!("{{\"name\":\"Amazon\",\"metadata\":{}}}", sample_meta_data())
    }
}

