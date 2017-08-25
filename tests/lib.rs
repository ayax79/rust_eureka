extern crate tokio_core;
extern crate futures;
extern crate rust_eureka;
extern crate serde_json;

//#[macro_use]
//extern crate log;
#[macro_use]
extern crate test_logger;


use std::env::var;
use tokio_core::reactor::Core;
use rust_eureka::EurekaClient;
use rust_eureka::request::{RegisterRequest, Instance, Status, DataCenterInfo, DcName, AmazonMetaData};
use serde_json::Map;
use std::{thread, time};


const EUREKA_URI_KEY: &'static str = "EUREKA_URI";
const EUREKA_CLIENT: &'static str = "INTEGRATION_TEST";


test!(test_register, {
    if let Some(eureka_uri) = get_eureka_uri() {
        let request = build_test_register_request();

        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = EurekaClient::new(&handle, EUREKA_CLIENT, eureka_uri.as_ref());
        let register = client.register(EUREKA_URI_KEY, &request);
        
        let ten_secs = time::Duration::from_secs(10);

        thread::sleep(ten_secs);

        let result = core.run(register);
        println!("result: {:?}", result);
        assert!(result.is_ok());
        let query = client.get_application_instances(EUREKA_CLIENT);
        let result = core.run(query);
        println!("result {:?} ", result);
        assert!(result.is_ok());

    }
    else {
        println!("Skipping test_register as there is no eureka uri specified")
    }
});

#[test]
fn output_json() {
    let request = build_test_register_request();
    let json = serde_json::to_string(&request);
    println!("{:?}", json.unwrap());
}


fn build_test_register_request() -> RegisterRequest {
    RegisterRequest::new(
        Instance {
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
                    instance_type: "SomeType".to_owned()
                })
            },
            lease_info: None,
            metadata: Map::new()
        })
}

fn get_eureka_uri() -> Option<String> {
    var(EUREKA_URI_KEY).ok()
}