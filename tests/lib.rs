use httpmock::MockServer;
use rust_eureka::request::{
    AmazonMetaData, DataCenterInfo, DcName, Instance, RegisterRequest, Status,
};
use rust_eureka::EurekaClient;
use serde_json::Map;

const EUREKA_CLIENT: &str = "INTEGRATION_TEST";

#[tokio::test]
async fn test_register_with_mock_server() {
    // Use httpmock to provide a local mock Eureka server
    let server = MockServer::start_async().await;

    // Mock the XML registration endpoint - match any apps registration POST
    let m1 = server
        .mock_async(|when, then| {
            when.method("POST")
                .path_includes("/apps/")
                .header("content-type", "application/xml");
            then.status(204);
        })
        .await;

    // Build client pointed at mock server
    let base = &server.base_url();
    let client = EurekaClient::new(EUREKA_CLIENT, base).unwrap();

    // Build a minimal register request
    let request = build_test_register_request();

    // Try register - should be Ok (we return 204 in mock)
    let result = client.register(EUREKA_CLIENT, &request).await;
    assert!(result.is_ok());

    m1.assert_async().await;
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
