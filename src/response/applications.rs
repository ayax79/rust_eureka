use super::Application;
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess, SeqAccess};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::convert::From;
use std::fmt;


#[derive(Debug, PartialEq, Deserialize)]
pub struct Applications {
    #[serde(rename = "versions__delta")]
    pub versions_delta: i16,
    #[serde(rename = "apps__hashcode")]
    pub apps_hashcode: String,
    #[serde(rename = "application", deserialize_with = "deserialize_applications_field")]
    pub applications: Vec<Application>
}


impl Serialize for Applications {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct("Applications", 3)?;
        s.serialize_field("versions__delta", &self.versions_delta)?;
        s.serialize_field("apps__hashcode", &self.apps_hashcode)?;

        if self.applications.len() == 1 {
            s.serialize_field("application", &self.applications.get(0))?;
        } else if !self.applications.is_empty() {
            s.serialize_field("application", &self.applications)?;
        }
        s.end()
    }
}

impl From<Application> for Vec<Application> {
    fn from(v: Application) -> Self {
        vec![v]
    }
}

fn deserialize_applications_field<'de, D>(de: D) -> Result<Vec<Application>, D::Error>
    where D: Deserializer<'de> {
    struct ApplicationOrVec;

    impl<'de> Visitor<'de> for ApplicationOrVec {
        type Value = Vec<Application>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("application or vec")
        }

        fn visit_map<A>(self, visitor: A) -> Result<Self::Value, A::Error> where
            A: MapAccess<'de>, {
            let result: Result<Application, A::Error> = Deserialize::deserialize(de::value::MapAccessDeserializer::new(visitor));
            result.map(|a| From::from(a))
        }

        fn visit_seq<A>(self, visitor: A) -> Result<Self::Value, A::Error> where
            A: SeqAccess<'de>, {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    de.deserialize_any(ApplicationOrVec)
}

#[cfg(test)]
pub mod tests {
    use serde_json::{self, Map};
    use super::*;
    use super::super::Application;
    use super::super::Instance;
    use super::super::Status;
    use super::super::DataCenterInfo;
    use super::super::DcName;
    use super::super::LeaseInfo;
    use super::super::ActionType;

    #[test]
    fn test_applications_serialize() {
        let applications = build_test_applications();
        let result = serde_json::to_string(&applications).unwrap();
        assert!(result.contains("\"apps__hashcode\":\"UP_1_\""));
        assert!(result.contains("\"name\":\"INTEGRATION_TEST\""));
    }

    #[test]
    fn test_applications_deserialize() {
        let json = build_test_applications_json();
        let applications = build_test_applications();
        let result = serde_json::from_str(json.as_ref()).unwrap();

        assert_eq!(applications, result)
    }

    #[test]
    fn test_applications_multi_deserialize() {
        let json = build_test_multi_applications_json();
        let result: Applications = serde_json::from_str(json.as_ref()).unwrap();
        assert_eq!(2, result.applications.len())
    }

    pub fn build_test_applications() -> Applications {
        Applications {
            versions_delta: 1,
            apps_hashcode: "UP_1_".to_string(),
            applications: vec![
                Application {
                    name: "INTEGRATION_TEST".to_string(),
                    instance: Instance {
                        host_name: "localhost".to_string(),
                        app: "INTEGRATION_TEST".to_string(),
                        ip_addr: "127.0.0.1".to_string(),
                        status: Status::Up,
                        overriddenstatus: Some(Status::Unknown),
                        port: Some(7001),
                        secure_port: Some(7002),
                        country_id: 1,
                        data_center_info: DataCenterInfo {
                            name: DcName::MyOwn,
                            metadata: None
                        },
                        lease_info: Some(LeaseInfo {
                            renewal_interval_in_secs: 30,
                            duration_in_secs: 90,
                            registration_timestamp: 1503701416749,
                            last_renewal_timestamp: 1503701416749,
                            eviction_timestamp: 0,
                            service_up_timestamp: 1503701416464
                        }),
                        metadata: Map::new(),
                        homepage_url: "http://google.com".to_string(),
                        status_page_url: "http://google.com".to_string(),
                        health_check_url: "http://google.com".to_string(),
                        vip_address: "127.0.0.1".to_string(),
                        secure_vip_address: "127.0.0.1".to_string(),
                        is_coordinating_discovery_server: false,
                        last_updated_timestamp: 1503701416750,
                        last_dirty_timestamp: 1503701416457,
                        action_type: ActionType::Added
                    }
                }
            ]
        }
    }

    pub fn build_test_applications_json() -> String {
        r#"
        {
        "versions__delta": 1,
        "apps__hashcode": "UP_1_",
        "application": {
            "name": "INTEGRATION_TEST",
            "instance": {
                "hostName": "localhost",
                "app": "INTEGRATION_TEST",
                "ipAddr": "127.0.0.1",
                "status": "UP",
                "overriddenstatus": "UNKNOWN",
                "port": {
                    "@enabled": "true",
                    "$": "7001"
                },
                "securePort": {
                    "@enabled": "false",
                    "$": "7002"
                },
                "countryId": 1,
                "dataCenterInfo": {
                    "name": "MyOwn"
                },
                "leaseInfo": {
                    "renewalIntervalInSecs": 30,
                    "durationInSecs": 90,
                    "registrationTimestamp": 1503701416749,
                    "lastRenewalTimestamp": 1503701416749,
                    "evictionTimestamp": 0,
                    "serviceUpTimestamp": 1503701416464
                },
                "metadata": {
                    "@class": "java.util.Collections$EmptyMap"
                },
                "homePageUrl": "http://google.com",
                "statusPageUrl": "http://google.com",
                "healthCheckUrl": "http://google.com",
                "vipAddress": "127.0.0.1",
                "secureVipAddress": "127.0.0.1",
                "isCoordinatingDiscoveryServer": false,
                "lastUpdatedTimestamp": 1503701416750,
                "lastDirtyTimestamp": 1503701416457,
                "actionType": "ADDED"
            }
        }
        }
        "#.to_string()
            .replace(" ", "")
            .replace("\n", "")
    }

    pub fn build_test_multi_applications_json() -> String {
        r#"
        {
        "versions__delta": 1,
        "apps__hashcode": "UP_1_",
        "application": [{
            "name": "INTEGRATION_TEST",
            "instance": {
                "hostName": "localhost",
                "app": "INTEGRATION_TEST",
                "ipAddr": "127.0.0.1",
                "status": "UP",
                "overriddenstatus": "UNKNOWN",
                "port": {
                    "@enabled": "true",
                    "$": "7001"
                },
                "securePort": {
                    "@enabled": "false",
                    "$": "7002"
                },
                "countryId": 1,
                "dataCenterInfo": {
                    "name": "MyOwn"
                },
                "leaseInfo": {
                    "renewalIntervalInSecs": 30,
                    "durationInSecs": 90,
                    "registrationTimestamp": 1503701416749,
                    "lastRenewalTimestamp": 1503701416749,
                    "evictionTimestamp": 0,
                    "serviceUpTimestamp": 1503701416464
                },
                "metadata": {
                    "@class": "java.util.Collections$EmptyMap"
                },
                "homePageUrl": "http://google.com",
                "statusPageUrl": "http://google.com",
                "healthCheckUrl": "http://google.com",
                "vipAddress": "127.0.0.1",
                "secureVipAddress": "127.0.0.1",
                "isCoordinatingDiscoveryServer": false,
                "lastUpdatedTimestamp": 1503701416750,
                "lastDirtyTimestamp": 1503701416457,
                "actionType": "ADDED"
            }
        }, {
            "name": "INTEGRATION_TEST2",
            "instance": {
                "hostName": "localhost",
                "app": "INTEGRATION_TEST",
                "ipAddr": "127.0.0.1",
                "status": "UP",
                "overriddenstatus": "UNKNOWN",
                "port": {
                    "@enabled": "true",
                    "$": "7001"
                },
                "securePort": {
                    "@enabled": "false",
                    "$": "7002"
                },
                "countryId": 1,
                "dataCenterInfo": {
                    "name": "MyOwn"
                },
                "leaseInfo": {
                    "renewalIntervalInSecs": 30,
                    "durationInSecs": 90,
                    "registrationTimestamp": 1503701416749,
                    "lastRenewalTimestamp": 1503701416749,
                    "evictionTimestamp": 0,
                    "serviceUpTimestamp": 1503701416464
                },
                "metadata": {
                    "@class": "java.util.Collections$EmptyMap"
                },
                "homePageUrl": "http://google.com",
                "statusPageUrl": "http://google.com",
                "healthCheckUrl": "http://google.com",
                "vipAddress": "127.0.0.1",
                "secureVipAddress": "127.0.0.1",
                "isCoordinatingDiscoveryServer": false,
                "lastUpdatedTimestamp": 1503701416750,
                "lastDirtyTimestamp": 1503701416457,
                "actionType": "ADDED"
            }
        }]
        }
        "#.to_string()
            .replace(" ", "")
            .replace("\n", "")
    }
}
