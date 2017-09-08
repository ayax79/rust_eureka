use super::Application;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationResponse {
    pub application: Application
}

impl<'a> ApplicationResponse {
    pub fn new(application: Application) -> ApplicationResponse {
        ApplicationResponse {
            application: application
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;
    use super::super::instance::tests::{build_test_instance, build_test_instance_json};

    #[test]
    fn test_application_response_serialization() {
        let json = build_application_response_json();
        let instance = build_test_instance();
        let application: Application = Application {
            name: "test_app".to_owned(),
            instance: instance
        };
        let ar = ApplicationResponse::new(application);
        let result = serde_json::to_string(&ar).unwrap();

        //                let combined = json.chars().zip(result.chars());
        //                for (a, b) in combined {
        //                    print!("{}", b);
        //                    assert_eq!(a, b);
        //                }
        assert_eq!(json, result);
    }

    #[test]
    fn test_application_response_deserialization() {
        let json = build_application_response_json();
        let instance = build_test_instance();
        let application: Application = Application {
            name: "test_app".to_owned(),
            instance: instance
        };
        let ar = ApplicationResponse::new(application);
        let result = serde_json::from_str(&json).unwrap();
        assert_eq!(ar, result);
    }

    fn build_application_response_json() -> String {

        format!("{{\"application\":{{\"name\":\"test_app\",\"instance\":{}}}}}", build_test_instance_json())
    }
}

