use rust_eureka::request::{
    AmazonMetaData, DataCenterInfo, DcName, Instance, RegisterRequest, Status,
};
use rust_eureka::EurekaClient;
use serde_json::Map;
use std::env::var;
use std::{thread, time};

const EUREKA_URI_KEY: &str = "EUREKA_URI";
const EUREKA_CLIENT: &str = "INTEGRATION_TEST";

#[tokio::test]
async fn test_register() {
    if let Some(eureka_uri) = get_eureka_uri() {
        let request = build_test_register_request();
        let client = EurekaClient::new(EUREKA_CLIENT, eureka_uri.as_ref());

        println!("#### Registering");
        let result = client.register(EUREKA_URI_KEY, &request).await;
        println!("result: {:?}", result);
        assert!(result.is_ok());

        let ten_secs = time::Duration::from_secs(10);
        thread::sleep(ten_secs);

        println!("#### Querying single application");
        let result = client.get_application(EUREKA_CLIENT).await;
        println!("result {:?} ", result);
        assert!(result.is_ok());

        println!("#### Querying multiple applications");
        let result = client.get_applications().await;
        println!("result {:?} ", result);
        assert!(result.is_ok());
    } else {
        println!("Skipping test_register as there is no eureka uri specified")
    }
}

#[test]
fn output_json() {
    let request = build_test_register_request();
    let json = serde_json::to_string(&request);
    println!("{:?}", json.unwrap());
}

fn build_test_register_request() -> RegisterRequest {
    RegisterRequest::new(Instance {
        host_name: "localhost".to_owned(),
        app: EUREKA_CLIENT.to_owned(),
        ip_addr: "127.0.0.1".to_owned(),
        vip_address: "127.0.0.1".to_owned(),
        secure_vip_address: "127.0.0.1".to_owned(),
        status: Status::Up,
        port: None,
        secure_port: None,
        homepage_url: "http://google.com".to_owned(),
        status_page_url: "http://google.com".to_owned(),
        health_check_url: "http://google.com".to_owned(),
        data_center_info: DataCenterInfo {
            name: DcName::MyOwn,
            metadata: Some(AmazonMetaData {
                ami_launch_index: "001".to_owned(),
                local_hostname: "localhost".to_owned(),
                availability_zone: "N/A".to_owned(),
                instance_id: "001".to_owned(),
                public_ip4: "127.0.0.1".to_owned(),
                public_hostname: "localhost".to_owned(),
                ami_manifest_path: "/a/path".to_owned(),
                local_ip4: "127.0.0.1".to_owned(),
                hostname: "localhost".to_owned(),
                ami_id: "232332".to_owned(),
                instance_type: "SomeType".to_owned(),
            }),
        },
        lease_info: None,
        metadata: Map::new(),
    })
}

fn get_eureka_uri() -> Option<String> {
    var(EUREKA_URI_KEY).ok()
}
