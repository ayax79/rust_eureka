use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError, MapAccess};
use std::fmt;
use super::DataCenterInfo;
use super::LeaseInfo;
use super::Status;

// Field name constants
const INSTANCE: &'static str = "Instance";
const HOST_NAME: &'static str = "hostName";
const APP: &'static str = "app";
const IP_ADDR: &'static str = "ipAddr";
const VIP_ADDRESS: &'static str = "vipAddress";
const SECURE_VIP_ADDRESS: &'static str = "secureVipAddress";
const STATUS: &'static str = "status";
const PORT: &'static str = "port";
const SECURE_PORT: &'static str = "securePort";
const HOME_PAGE_URL: &'static str = "homePageUrl";
const STATUS_PAGE_URL: &'static str = "statusPageUrl";
const HEALTH_CHECK_URL: &'static str = "healthCheckUrl";
const DATA_CENTER_INFO: &'static str = "dataCenterInfo";
const LEASE_INFO: &'static str = "leaseInfo";
const METADATA: &'static str = "metadata";
const JSON_FIELDS: &'static [&'static str] = &[INSTANCE, HOST_NAME, APP, IP_ADDR, VIP_ADDRESS, SECURE_VIP_ADDRESS,
    STATUS, PORT, SECURE_PORT, HOME_PAGE_URL, STATUS_PAGE_URL, HEALTH_CHECK_URL,
    DATA_CENTER_INFO, LEASE_INFO, METADATA];
const RUST_FIELDS: &'static [&'static str] = &["host_name", "app", "ip_addr", "vip_address", "secure_vip_address",
    "status", "port Option", "secure_port", "homepage_url", "status_page_url",
    "health_check_url", "data_center_info", "lease_info", "metadata"];

const PORT_DOLLAR: &'static str = "$";
const PORT_ENABLED: &'static str = "@enabled";
const PORT_FIELDS: &'static [&'static str] = &[PORT_DOLLAR, PORT_ENABLED];

#[derive(Debug, PartialEq)]
pub struct Instance {
    pub host_name: String,
    pub app: String,
    pub ip_addr: String,
    pub vip_address: String,
    pub secure_vip_address: String,
    pub status: Status,
    pub port: Option<u16>,
    pub secure_port: Option<u16>,
    pub homepage_url: String,
    pub status_page_url: String,
    pub health_check_url: String,
    pub data_center_info: DataCenterInfo,
    pub lease_info: Option<LeaseInfo>,
    pub metadata: Vec<String>
}

struct Port {
    port: u16
}

impl Port {
    fn new(port: u16) -> Port {
        Port { port: port }
    }
}

impl Serialize for Port {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct("Port", 2)?;
        s.serialize_field(PORT_DOLLAR, &self.port)?;
        s.serialize_field(PORT_ENABLED, &true)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Port {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        enum Field { DollarSign, Enabled };


        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
                D: Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("'$' or 'enabled'")
                    }
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                        E: DeError, {
                        match v {
                            PORT_DOLLAR => Ok(Field::DollarSign),
                            PORT_ENABLED => Ok(Field::Enabled),
                            _ => Err(DeError::unknown_field(v, PORT_FIELDS))
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct PortVisitor;
        impl<'de> Visitor<'de> for PortVisitor {
            type Value = Port;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Port")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where
                A: MapAccess<'de>, {
                let mut maybe_dollar: Option<u16> = None;
                let mut maybe_enabled: Option<bool> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::DollarSign => {
                            if maybe_dollar.is_some() {
                                return Err(DeError::duplicate_field(PORT_DOLLAR));
                            }
                            maybe_dollar = Some(map.next_value()?);
                        }
                        Field::Enabled => {
                            if maybe_enabled.is_some() {
                                return Err(DeError::duplicate_field(PORT_ENABLED));
                            }
                            maybe_enabled = Some(map.next_value()?);
                        }
                    }
                }

                let dollar = maybe_dollar.ok_or_else(|| DeError::missing_field(PORT_DOLLAR))?;
                maybe_enabled.ok_or_else(|| DeError::missing_field(PORT_ENABLED))?;
                // ignore enabled
                Ok(Port::new(dollar))
            }
        }

        deserializer.deserialize_struct("Port", PORT_FIELDS, PortVisitor)
    }
}

impl Serialize for Instance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct(INSTANCE, 14)?;
        s.serialize_field(HOST_NAME, &self.host_name)?;
        s.serialize_field(APP, &self.app)?;
        s.serialize_field(IP_ADDR, &self.ip_addr)?;
        s.serialize_field(VIP_ADDRESS, &self.vip_address)?;
        s.serialize_field(SECURE_VIP_ADDRESS, &self.secure_vip_address)?;
        s.serialize_field(STATUS, &self.status)?;

        if let &Some(p) = &self.port {
            let port = Port::new(p);
            s.serialize_field(PORT, &port)?;
        }

        if let &Some(p) = &self.secure_port {
            let port = Port::new(p);
            s.serialize_field(SECURE_PORT, &port)?;
        }

        s.serialize_field(HOME_PAGE_URL, &self.homepage_url)?;
        s.serialize_field(STATUS_PAGE_URL, &self.status_page_url)?;
        s.serialize_field(HEALTH_CHECK_URL, &self.health_check_url)?;
        s.serialize_field(DATA_CENTER_INFO, &self.data_center_info)?;

        if let &Some(ref lease_info) = &self.lease_info {
            s.serialize_field(LEASE_INFO, lease_info)?;
        }

        if !&self.metadata.is_empty() {
            s.serialize_field(METADATA, &self.metadata)?;
        }
        s.end()
    }
}

impl<'de> Deserialize<'de> for Instance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        enum Field {
            HostName,
            App,
            IpAddr,
            VipAddress,
            SecureVipAddress,
            Status,
            Port,
            SecurePort,
            HomepageUrl,
            StatusPageUrl,
            HealthCheckUrl,
            DataCenterInfo,
            LeaseInfo,
            Metadata
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
                D: Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("An Instance field (see schema)")
                    }
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                        E: DeError {
                        match v {
                            HOST_NAME => Ok(Field::HostName),
                            APP => Ok(Field::App),
                            IP_ADDR => Ok(Field::IpAddr),
                            VIP_ADDRESS => Ok(Field::VipAddress),
                            SECURE_VIP_ADDRESS => Ok(Field::SecureVipAddress),
                            STATUS => Ok(Field::Status),
                            PORT => Ok(Field::Port),
                            SECURE_PORT => Ok(Field::SecurePort),
                            HOME_PAGE_URL => Ok(Field::HomepageUrl),
                            STATUS_PAGE_URL => Ok(Field::StatusPageUrl),
                            HEALTH_CHECK_URL => Ok(Field::HealthCheckUrl),
                            DATA_CENTER_INFO => Ok(Field::DataCenterInfo),
                            LEASE_INFO => Ok(Field::LeaseInfo),
                            METADATA => Ok(Field::Metadata),
                            _ => Err(DeError::unknown_field(v, JSON_FIELDS))
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct InstanceVisitor;

        impl<'de> Visitor<'de> for InstanceVisitor {
            type Value = Instance;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Instance")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where
                A: MapAccess<'de> {
                let mut maybe_host_name = None;
                let mut maybe_app = None;
                let mut maybe_ip_addr = None;
                let mut maybe_vip_address = None;
                let mut maybe_secure_vip_address = None;
                let mut maybe_status = None;
                let mut maybe_port: Option<Port> = None;
                let mut maybe_secure_port: Option<Port> = None;
                let mut maybe_homepage_url = None;
                let mut maybe_status_page_url = None;
                let mut maybe_health_check_url = None;
                let mut maybe_data_center_info = None;
                let mut maybe_lease_info = None;
                let mut maybe_metadata = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::HomepageUrl => {
                            if maybe_homepage_url.is_some() {
                                return Err(DeError::duplicate_field(HOME_PAGE_URL));
                            }
                            maybe_homepage_url = Some(map.next_value()?);
                        }
                        Field::App => {
                            if maybe_app.is_some() {
                                return Err(DeError::duplicate_field(APP));
                            }
                            maybe_app = Some(map.next_value()?);
                        }
                        Field::IpAddr => {
                            if maybe_ip_addr.is_some() {
                                return Err(DeError::duplicate_field(IP_ADDR));
                            }
                            maybe_ip_addr = Some(map.next_value()?);
                        }
                        Field::VipAddress => {
                            if maybe_vip_address.is_some() {
                                return Err(DeError::duplicate_field(VIP_ADDRESS));
                            }
                            maybe_vip_address = Some(map.next_value()?);
                        }
                        Field::SecureVipAddress => {
                            if maybe_secure_vip_address.is_some() {
                                return Err(DeError::duplicate_field(SECURE_VIP_ADDRESS));
                            }
                            maybe_secure_vip_address = Some(map.next_value()?);
                        }
                        Field::Status => {
                            if maybe_status.is_some() {
                                return Err(DeError::duplicate_field(STATUS));
                            }
                            maybe_status = Some(map.next_value()?);
                        }
                        Field::Port => {
                            if maybe_port.is_some() {
                                return Err(DeError::duplicate_field(PORT));
                            }
                            maybe_port = Some(map.next_value()?);
                        }
                        Field::SecurePort => {
                            if maybe_secure_port.is_some() {
                                return Err(DeError::duplicate_field(SECURE_PORT));
                            }
                            maybe_secure_port = Some(map.next_value()?);
                        }
                        Field::StatusPageUrl => {
                            if maybe_status_page_url.is_some() {
                                return Err(DeError::duplicate_field(STATUS_PAGE_URL));
                            }
                            maybe_status_page_url = Some(map.next_value()?);
                        }
                        Field::HealthCheckUrl => {
                            if maybe_health_check_url.is_some() {
                                return Err(DeError::duplicate_field(HEALTH_CHECK_URL));
                            }
                            maybe_health_check_url = Some(map.next_value()?);
                        }
                        Field::DataCenterInfo => {
                            if maybe_data_center_info.is_some() {
                                return Err(DeError::duplicate_field(DATA_CENTER_INFO));
                            }
                            maybe_data_center_info = Some(map.next_value()?);
                        }
                        Field::LeaseInfo => {
                            if maybe_lease_info.is_some() {
                                return Err(DeError::duplicate_field(LEASE_INFO));
                            }
                            maybe_lease_info = Some(map.next_value()?);
                        }
                        Field::Metadata => {
                            if maybe_metadata.is_some() {
                                return Err(DeError::duplicate_field(METADATA));
                            }
                            maybe_metadata = Some(map.next_value()?);
                        }
                        Field::HostName => {
                            if maybe_host_name.is_some() {
                                return Err(DeError::duplicate_field(HOST_NAME));
                            }
                            maybe_host_name = Some(map.next_value()?);
                        }
                    }
                }

                let host_name = maybe_host_name.ok_or_else(|| DeError::missing_field(HOST_NAME));
                let app = maybe_app.ok_or_else(|| DeError::missing_field(APP));
                let ip_addr = maybe_ip_addr.ok_or_else(|| DeError::missing_field(IP_ADDR));
                let vip_address = maybe_vip_address.ok_or_else(|| DeError::missing_field(VIP_ADDRESS));
                let secure_vip_address = maybe_secure_vip_address.ok_or_else(|| DeError::missing_field(SECURE_VIP_ADDRESS));
                let status = maybe_status.ok_or_else(|| DeError::missing_field(STATUS));
                let homepage_url = maybe_homepage_url.ok_or_else(|| DeError::missing_field(HOME_PAGE_URL));
                let status_page_url = maybe_status_page_url.ok_or_else(|| DeError::missing_field(STATUS_PAGE_URL));
                let health_check_url = maybe_health_check_url.ok_or_else(|| DeError::missing_field(HEALTH_CHECK_URL));
                let data_center_info = maybe_data_center_info.ok_or_else(|| DeError::missing_field(DATA_CENTER_INFO));
                let metadata = maybe_metadata.ok_or_else(|| DeError::missing_field(METADATA));

                Ok(Instance {
                    host_name: host_name?,
                    app: app?,
                    ip_addr: ip_addr?,
                    vip_address: vip_address?,
                    secure_vip_address: secure_vip_address?,
                    status: status?,
                    port: maybe_port.map(|p| p.port),
                    secure_port: maybe_secure_port.map(|p| p.port),
                    homepage_url: homepage_url?,
                    status_page_url: status_page_url?,
                    health_check_url: health_check_url?,
                    data_center_info: data_center_info?,
                    lease_info: maybe_lease_info,
                    metadata: metadata?
                })
            }
        }
        deserializer.deserialize_struct(INSTANCE, RUST_FIELDS, InstanceVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use super::super::DcName;
    use super::super::AmazonMetaData;

    #[test]
    fn test_instance_serialization() {
        let json = build_test_json();
        let instance = build_test_instance();
        let result = serde_json::to_string(&instance).unwrap();

        //        let combined = json.chars().zip(result.chars());
        //        for (a, b) in combined {
        //            print!("{}", b);
        //            assert_eq!(a, b);
        //        }
        assert_eq!(json, result);
    }

    #[test]
    fn test_instance_deserialization() {
        let json = build_test_json();
        let instance = build_test_instance();
        let result = serde_json::from_str(&json).unwrap();
        assert_eq!(instance, result);
    }

    fn build_test_json() -> String {
        r#"{
           "hostName": "Foo",
           "app": "Bar",
           "ipAddr": "3.128.2.12",
           "vipAddress": "127.0.0.1",
           "secureVipAddress": "127.0.0.2",
           "status": "UP",
           "port": { "$": 80, "@enabled": true },
           "securePort": { "$": 443, "@enabled": true },
           "homePageUrl": "http://google.com",
           "statusPageUrl": "http://nytimes.com",
           "healthCheckUrl": "http://washingtonpost.com",
           "dataCenterInfo": { "@class": "com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo", "name":"Amazon","metadata":
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
           "metadata": ["something", "somethingelse"]
        }"#
            .to_string()
            .replace(" ", "")
            .replace("\n", "")
    }

    fn build_test_instance() -> Instance {
        Instance {
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
            metadata: vec!["something".to_string(), "somethingelse".to_string()]
        }
    }
}

