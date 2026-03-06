use super::Application;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Applications {
    pub versions_delta: i16,
    pub apps_hashcode: String,
    pub applications: Vec<Application>,
}

// Custom deserializer to handle both single Application object and array of Applications
fn deserialize_application_field<'de, D>(deserializer: D) -> Result<Vec<Application>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ApplicationOrVec;

    impl<'de> Visitor<'de> for ApplicationOrVec {
        type Value = Vec<Application>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("application or array of applications")
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let app: Application =
                Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(vec![app])
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(ApplicationOrVec)
}

// Manual implementation of Serialize for Applications
impl Serialize for Applications {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Applications", 3)?;
        state.serialize_field("versions__delta", &self.versions_delta)?;
        state.serialize_field("apps__hashcode", &self.apps_hashcode)?;
        // Serialize as single object if only one application, array if multiple
        if self.applications.len() == 1 {
            state.serialize_field("application", &self.applications[0])?;
        } else {
            state.serialize_field("application", &self.applications)?;
        }
        state.end()
    }
}

// Manual implementation of Deserialize for Applications
impl<'de> Deserialize<'de> for Applications {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationsVisitor;

        impl<'de> Visitor<'de> for ApplicationsVisitor {
            type Value = Applications;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Applications")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Applications, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut versions_delta = None;
                let mut apps_hashcode = None;
                let mut applications = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "versions__delta" => {
                            if versions_delta.is_some() {
                                return Err(de::Error::duplicate_field("versions__delta"));
                            }
                            versions_delta = Some(map.next_value()?);
                        }
                        "apps__hashcode" => {
                            if apps_hashcode.is_some() {
                                return Err(de::Error::duplicate_field("apps__hashcode"));
                            }
                            apps_hashcode = Some(map.next_value()?);
                        }
                        "application" => {
                            if applications.is_some() {
                                return Err(de::Error::duplicate_field("application"));
                            }
                            applications = Some(map.next_value_seed(ApplicationFieldSeed)?);
                        }
                        _ => {
                            // Skip unknown fields
                            let _: de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let versions_delta =
                    versions_delta.ok_or_else(|| de::Error::missing_field("versions__delta"))?;
                let apps_hashcode =
                    apps_hashcode.ok_or_else(|| de::Error::missing_field("apps__hashcode"))?;
                let applications =
                    applications.ok_or_else(|| de::Error::missing_field("application"))?;

                Ok(Applications {
                    versions_delta,
                    apps_hashcode,
                    applications,
                })
            }
        }

        struct ApplicationFieldSeed;

        impl<'de> de::DeserializeSeed<'de> for ApplicationFieldSeed {
            type Value = Vec<Application>;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserialize_application_field(deserializer)
            }
        }

        const FIELDS: &[&str] = &["versions__delta", "apps__hashcode", "application"];
        deserializer.deserialize_struct("Applications", FIELDS, ApplicationsVisitor)
    }
}

impl From<Application> for Vec<Application> {
    fn from(v: Application) -> Self {
        vec![v]
    }
}

#[cfg(test)]
pub mod tests {
    use super::super::ActionType;
    use super::super::Application;
    use super::super::DataCenterInfo;
    use super::super::DcName;
    use super::super::Instance;
    use super::super::LeaseInfo;
    use super::super::Status;
    use super::*;
    use serde_json::{self, Map};

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
            applications: vec![Application {
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
                        metadata: None,
                    },
                    lease_info: Some(LeaseInfo {
                        renewal_interval_in_secs: 30,
                        duration_in_secs: 90,
                        registration_timestamp: 1503701416749,
                        last_renewal_timestamp: 1503701416749,
                        eviction_timestamp: 0,
                        service_up_timestamp: 1503701416464,
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
                    action_type: ActionType::Added,
                },
            }],
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
        "#
        .to_string()
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
        "#
        .to_string()
        .replace(" ", "")
        .replace("\n", "")
    }
}
