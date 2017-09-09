use super::Applications;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationsResponse {
    pub applications: Applications
}

impl<'a> ApplicationsResponse {
    pub fn new(applications: Applications) -> ApplicationsResponse {
        ApplicationsResponse {
            applications: applications
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;
    use super::super::applications::tests::{build_test_applications, build_test_applications_json};

    #[test]
    fn test_applications_response_serialization() {
        let applications = build_test_applications();
        let ar = ApplicationsResponse::new(applications);
        let result = serde_json::to_string(&ar).unwrap();
        assert!(result.contains("{\"applications\":"))

    }

    #[test]
    fn test_applications_response_deserialization() {
        let json = build_applications_response_json();
        let applications = build_test_applications();
        let ar = ApplicationsResponse::new(applications);
        let result = serde_json::from_str(&json).unwrap();
        assert_eq!(ar, result);
    }

    #[test]
    fn test_local_eureka_response() {
        let ar = serde_json::from_str::<ApplicationsResponse>(local_eureka_json().as_ref());
        assert!(ar.is_ok())
    }

    fn build_applications_response_json() -> String {
        format!("{{\"applications\":{}}}", build_test_applications_json())
    }

    fn local_eureka_json() -> String {
        r#"{
    "applications": {
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
                    "@class": "com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo",
                    "name": "MyOwn"
                },
                "leaseInfo": {
                    "renewalIntervalInSecs": 30,
                    "durationInSecs": 90,
                    "registrationTimestamp": 1504830481334,
                    "lastRenewalTimestamp": 1504830481334,
                    "evictionTimestamp": 0,
                    "serviceUpTimestamp": 1504830302194
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
                "lastUpdatedTimestamp": 1504830481334,
                "lastDirtyTimestamp": 1504830480933,
                "actionType": "ADDED"
            }
        }
    }
}"#.to_string()
    }
}

