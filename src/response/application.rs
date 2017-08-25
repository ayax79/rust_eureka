use super::Instance;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    pub instance: Instance
}


#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;
    use super::super::instance::tests::{build_test_instance, build_test_instance_json};

    #[test]
    fn test_instance_serialization() {
        let json = build_register_json();
        let instance = build_test_instance();
        let name = "test_name";
        let app = Application{
            name: name.to_owned(),
            instance: instance
        };
        let result = serde_json::to_string(&app).unwrap();

        //                let combined = json.chars().zip(result.chars());
        //                for (a, b) in combined {
        //                    print!("{}", b);
        //                    assert_eq!(a, b);
        //                }
        assert_eq!(json, result);
    }

    #[test]
    fn test_instance_deserialization() {
        let json = build_register_json();
        let instance = build_test_instance();
        let name = "test_name";
        let app = Application{
            name: name.to_owned(),
            instance: instance
        };
        let result = serde_json::from_str(&json).unwrap();
        assert_eq!(app, result);
    }

    fn build_register_json() -> String {
        format!("{{\"name\":\"test_name\",\"instance\":{}}}", build_test_instance_json())
    }
}

