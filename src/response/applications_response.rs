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

    fn build_applications_response_json() -> String {
        format!("{{\"applications\":{}}}", build_test_applications_json())
    }
}

