#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaseInfo {
    pub renewal_interval_in_secs: i64,
    pub duration_in_secs: i64,
    pub registration_timestamp: i64,
    pub last_renewal_timestamp: i64,
    pub eviction_timestamp: i64,
    pub service_up_timestamp: i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize(){
        let li = build_lease_info();
        let json = build_lease_info_json();
        let result = serde_json::to_string(&li).unwrap();
        assert_eq!(json, result);
    }

    #[test]
    fn test_deserialize() {
        let li = build_lease_info();
        let json = build_lease_info_json();
        let result = serde_json::from_str(json.as_ref()).unwrap();
        assert_eq!(li, result);
    }


    fn build_lease_info_json() -> String {
        r#"{
            "renewalIntervalInSecs": 30,
            "durationInSecs": 90,
            "registrationTimestamp": 1503442035871,
            "lastRenewalTimestamp": 1503442035871,
            "evictionTimestamp": 0,
            "serviceUpTimestamp": 1503442035721
        }"#
            .to_string()
            .replace(" ", "")
            .replace("\n", "")
    }

    fn build_lease_info() -> LeaseInfo {
        LeaseInfo {
            renewal_interval_in_secs: 30,
            duration_in_secs: 90,
            registration_timestamp: 1503442035871,
            last_renewal_timestamp: 1503442035871,
            eviction_timestamp: 0,
            service_up_timestamp: 1503442035721,
        }
    }
}
