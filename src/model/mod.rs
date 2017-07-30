mod status;
mod dcname;
mod amazonmetadata;
mod datacenterinfo;
mod leaseinfo;

pub use self::status::Status;
pub use self::dcname::DcName;
pub use self::amazonmetadata::AmazonMetaData;
pub use self::datacenterinfo::DataCenterInfo;
pub use self::leaseinfo::LeaseInfo;

use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError};
use std::iter::Iterator;
use std::fmt;
use std::ops::Add;
use std::convert::From;
use std::str::FromStr;


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

}

