use serde::ser::{Serialize, Serializer, SerializeStruct};

#[allow(dead_code)]
pub struct Instance {
    host_name: String,
    app: String,
    ip_addr: String,
    vip_address: String,
    secure_vip_address: String,
    status: Status,
    port: Option<u16>,
    secure_port: u16,
    homepage_url: String,
    status_page_url: String,
    health_check_url: String,
    data_center_info: DataCenterInfo,
    lease_info: Option<LeaseInfo>,
    metadata: Vec<String>
}



#[allow(dead_code)]
pub enum Status {
    Up,
    Down,
    Starting,
    OutOfService,
    Unknown
}

#[allow(dead_code)]
impl ToString for Status {

    fn to_string(&self) -> String {
        let result = match self {
            &Status::Up => "UP",
            &Status::Down => "DOWN",
            &Status::Starting => "STARTING",
            &Status::OutOfService => "OUT_OF_SERVICE",
            _ => "UNKNOWN"
        };
        result.to_string()
    }

}

#[allow(dead_code)]
pub enum DcName {
    MyOwn,
    Amaon
}

#[allow(dead_code)]
pub struct DataCenterInfo {
    name: DcName,
    metadata: AmazonMetaData
}

#[allow(dead_code)]
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


#[allow(dead_code)]
pub struct LeaseInfo {
    eviction_duration_in_secs: Option<u32>
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

//    #[test]
//    fn test_instance() {
//        let instance = Instance {
//            host_name: "Foo".to_string(),
//            app: "Bar".to_string(),
//            ip_addr: "3.128.2.12".to_string(),
//            vip_address: "127.0.0.1".to_string(),
//            secure_vip_address: "127.0.0.2".to_string(),
//            status: Status::Up,
//            port: Some(80),
//            secure_port: Some(443),
//            homepage_url: "http://google.com".to_string(),
//            status_page_url: "http://nytimes.com".to_string(),
//            health_check_url: "http://washingtonpost.com".to_string(),
//            data_center_info: DataCenterInfo {
//                name: DcName::Amaon,
//                metadata: AmazonMetaData {
//                    ami_launch_index: "001".to_string(),
//                    local_hostname: "localhost".to_string(),
//                    availability_zone: "US_East1".to_string(),
//                    instance_id: "instance1".to_string(),
//                    public_ip4: "32.23.21.21".to_string(),
//                    public_hostname: "foo.com".to_string(),
//                    ami_manifest_path: "/dev/null".to_string(),
//                    local_ip4: "127.0.0.1".to_string(),
//                    hostname: "privatefoo.com".to_string(),
//                    ami_id: "ami002".to_string(),
//                    instance_type: "c4xlarge".to_string()
//                }
//            },
//            lease_info: LeaseInfo {
//                eviction_duration_in_secs: 122121
//            },
//            metadata: vec!["something".to_string()]
//        };
//    }


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
        let json = r#"{"ami-launch-index":"001a","local-hostname":"localhost0","availability-zone":"US_East1a","instance-id":"instance1a","public-ipv4":"32.23.21.212","public-hostname":"foo.coma","ami-manifest-path":"/dev/nulla","local-ipv4":"127.0.0.12","hostname":"privatefoo.coma","ami-id":"ami0023","instance-type":"c4xlarged"}"#;

        let result = serde_json::to_string(&md).unwrap();
        assert_eq!(json, result);
    }
}

