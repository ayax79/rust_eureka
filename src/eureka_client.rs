use http::{http_get};
use serde_json;
use serde_json::Value;
use std::iter::FromIterator;

pub struct EurekaClient {
    client_name: String,
    eureka_cluster_url: String
}

//
// A simple port of the example found at: https://github.com/Netflix/eureka/wiki/Example-Custom-ReadOnly-client
// Eureka REST API: https://github.com/Netflix/eureka/wiki/Eureka-REST-operations
impl EurekaClient {

    pub fn new(client_name: &str, eureka_cluster_url: &str) -> EurekaClient {
        EurekaClient {
            client_name: client_name.to_string(),
            eureka_cluster_url: eureka_cluster_url.to_string()
        }
    }

    pub fn get_application_hosts(&self, application_name: &str) -> Vec<String> {
        let uri = self.eureka_cluster_url.clone() + application_name;
        let response = http_get(self.client_name.as_ref(), uri.as_ref()).unwrap();
        let json = serde_json::from_str(response.as_ref()).unwrap();
        self.extract_results(json)
    }

    fn extract_results(&self, json: Value) -> Vec<String> {
        // filter that can be usd on an instance
        // that only allows hostnames with a status of UP
        fn up_filter(v: &Value) -> bool {
            v.get("status")
                .and_then(|s| s.as_str())
                .map(|s| s == "UP")
                .unwrap_or(false)
        }

        let empty_results: Vec<Value> = Vec::new();

        // extract the application instances
        let instances: &Vec<Value> = json.get("application")
            .and_then(|app| app.get("instance"))
            .and_then(|inst| inst.as_array())
            .unwrap_or(empty_results.as_ref());

        // extract the host names that are UP
        let hosts = instances
            .into_iter()
            .filter(|ref to_filter| up_filter(to_filter))
            .flat_map(|ref instance| instance.get("hostName"))
            .flat_map(|h| h.as_str())
            .map(|s| s.to_string());

        Vec::from_iter(hosts)
    }
}
