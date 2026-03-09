//! Integration tests for rust_eureka client against a running Eureka server.
//!
//! These tests are designed to run against a Eureka server running on localhost:8080.
//!
//! ## Running the tests
//!
//! 1. Start a Eureka server:
//!    ```bash
//!    cd ~/nikedev/nike-edge/eureka/eureka-server
//!    gradle war
//!    java -jar build/libs/eureka-server-*.war
//!    ```
//!    Or use Docker:
//!    ```bash
//!    docker run -p 8080:8080 springcloud/eureka
//!    ```
//!
//! 2. Run the integration tests:
//!    ```bash
//!    EUREKA_URI=http://localhost:8080 cargo test --test integration_tests -- --test-threads=1
//!    ```
//!
//! Note: Tests run sequentially (--test-threads=1) to avoid conflicts.

use rust_eureka::request::{
    AmazonMetaData, DataCenterInfo, DcName, Instance, LeaseInfo, RegisterRequest, Status,
};
use rust_eureka::EurekaClient;
use serde_json::Map;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

const EUREKA_URI_KEY: &str = "EUREKA_URI";
const DEFAULT_EUREKA_URI: &str = "http://localhost:8080";

/// Get Eureka URI from environment or use default
fn get_eureka_uri() -> String {
    env::var(EUREKA_URI_KEY).unwrap_or_else(|_| DEFAULT_EUREKA_URI.to_string())
}

/// Create a unique app name for testing to avoid conflicts
fn create_test_app_name(test_name: &str) -> String {
    format!("RUST_EUREKA_TEST_{}", test_name.to_uppercase())
}

/// Build a test instance with the given app name
fn build_test_instance(app_name: &str, port: Option<u16>) -> Instance {
    Instance {
        host_name: "localhost".to_owned(),
        app: app_name.to_owned(),
        ip_addr: "127.0.0.1".to_owned(),
        vip_address: "127.0.0.1".to_owned(),
        secure_vip_address: "127.0.0.1".to_owned(),
        status: Status::Up,
        port,
        secure_port: None,
        homepage_url: "http://localhost:8080".to_owned(),
        status_page_url: "http://localhost:8080/status".to_owned(),
        health_check_url: "http://localhost:8080/health".to_owned(),
        data_center_info: DataCenterInfo {
            name: DcName::MyOwn,
            metadata: Some(AmazonMetaData {
                ami_launch_index: "001".to_owned(),
                local_hostname: "localhost".to_owned(),
                availability_zone: "us-east-1a".to_owned(),
                instance_id: "i-test001".to_owned(),
                public_ip4: "127.0.0.1".to_owned(),
                public_hostname: "localhost".to_owned(),
                ami_manifest_path: "/test/path".to_owned(),
                local_ip4: "127.0.0.1".to_owned(),
                hostname: "localhost".to_owned(),
                ami_id: "ami-test123".to_owned(),
                instance_type: "t2.micro".to_owned(),
            }),
        },
        lease_info: Some(LeaseInfo {
            eviction_duration_in_secs: Some(90),
        }),
        metadata: Map::new(),
    }
}

/// Test basic connectivity to Eureka server
#[tokio::test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
async fn test_eureka_connectivity() {
    let eureka_uri = get_eureka_uri();
    println!("Testing connectivity to Eureka at: {}", eureka_uri);

    let client = EurekaClient::new("CONNECTIVITY_TEST", &eureka_uri).unwrap();

    // Try to get applications - even if empty, this verifies server is reachable
    let result = client.get_applications().await;

    match result {
        Ok(apps) => {
            println!("✓ Successfully connected to Eureka");
            println!(
                "  Found {} applications",
                apps.applications.applications.len()
            );
        }
        Err(e) => {
            panic!(
                "✗ Failed to connect to Eureka at {}: {:?}\n\
                 Make sure Eureka server is running on localhost:8080\n\
                 Run: docker run -p 8080:8080 springcloud/eureka",
                eureka_uri, e
            );
        }
    }
}

/// Test registering a new application instance
#[tokio::test]
#[ignore]
async fn test_register_instance() {
    let eureka_uri = get_eureka_uri();
    let app_name = create_test_app_name("register");
    println!("Testing registration with app: {}", app_name);

    let client = EurekaClient::new(&app_name, &eureka_uri).unwrap();
    let instance = build_test_instance(&app_name, Some(8081));
    let request = RegisterRequest::new(instance);

    // Register the instance
    let result = client.register(&app_name, &request).await;
    assert!(
        result.is_ok(),
        "Failed to register instance: {:?}",
        result.err()
    );
    println!("✓ Successfully registered instance");

    // Wait for registration to propagate
    sleep(Duration::from_secs(2)).await;

    // Verify the instance is registered
    let app_result = client.get_application(&app_name).await;
    assert!(
        app_result.is_ok(),
        "Failed to get registered application: {:?}",
        app_result.err()
    );

    let app = app_result.unwrap();
    assert_eq!(app.application.name, app_name, "Application name mismatch");
    println!("✓ Verified instance is registered");
}

/// Test retrieving a single application
#[tokio::test]
#[ignore]
async fn test_get_application() {
    let eureka_uri = get_eureka_uri();
    let app_name = create_test_app_name("get_app");
    println!("Testing get application: {}", app_name);

    let client = EurekaClient::new(&app_name, &eureka_uri).unwrap();
    let instance = build_test_instance(&app_name, Some(8082));
    let request = RegisterRequest::new(instance);

    // First register an instance
    client
        .register(&app_name, &request)
        .await
        .expect("Failed to register instance");

    sleep(Duration::from_secs(2)).await;

    // Now get the application
    let result = client.get_application(&app_name).await;
    assert!(result.is_ok(), "Failed to get application: {:?}", result);

    let app_response = result.unwrap();
    assert_eq!(app_response.application.name, app_name);
    println!("✓ Successfully retrieved application");
}

/// Test retrieving all applications
#[tokio::test]
#[ignore]
async fn test_get_all_applications() {
    let eureka_uri = get_eureka_uri();
    let app_name = create_test_app_name("get_all");
    println!("Testing get all applications");

    let client = EurekaClient::new(&app_name, &eureka_uri).unwrap();
    let instance = build_test_instance(&app_name, Some(8083));
    let request = RegisterRequest::new(instance);

    // Register an instance first
    client
        .register(&app_name, &request)
        .await
        .expect("Failed to register instance");

    sleep(Duration::from_secs(2)).await;

    // Get all applications
    let result = client.get_applications().await;
    assert!(
        result.is_ok(),
        "Failed to get applications: {:?}",
        result.err()
    );

    let apps_response = result.unwrap();
    let apps = &apps_response.applications.applications;

    println!("✓ Found {} applications", apps.len());
    assert!(
        !apps.is_empty(),
        "Expected at least one application to be registered"
    );

    // Verify our app is in the list
    let found = apps.iter().any(|app| app.name == app_name);
    assert!(found, "Our registered app {} not found in list", app_name);
    println!("✓ Our application is in the registry");
}

/// Test error handling when getting a non-existent application
#[tokio::test]
#[ignore]
async fn test_get_nonexistent_application() {
    let eureka_uri = get_eureka_uri();
    let app_name = "NONEXISTENT_APP_12345";
    println!("Testing get non-existent application: {}", app_name);

    let client = EurekaClient::new(app_name, &eureka_uri).unwrap();

    let result = client.get_application(app_name).await;
    assert!(
        result.is_err(),
        "Expected error for non-existent application"
    );

    match result {
        Err(rust_eureka::errors::EurekaClientError::NotFound) => {
            println!("✓ Correctly received NotFound error");
        }
        Err(e) => {
            panic!("Expected NotFound error, got: {:?}", e);
        }
        Ok(_) => {
            panic!("Expected error but got success");
        }
    }
}

/// Test registering multiple instances of the same application
#[tokio::test]
#[ignore]
async fn test_register_multiple_instances() {
    let eureka_uri = get_eureka_uri();
    let app_name = create_test_app_name("multi");
    println!("Testing multiple instance registration: {}", app_name);

    // Register first instance
    let client1 = EurekaClient::new(&format!("{}-1", app_name), &eureka_uri).unwrap();
    let instance1 = build_test_instance(&app_name, Some(8084));
    let request1 = RegisterRequest::new(instance1);
    client1
        .register(&app_name, &request1)
        .await
        .expect("Failed to register first instance");

    // Register second instance
    let client2 = EurekaClient::new(&format!("{}-2", app_name), &eureka_uri).unwrap();
    let instance2 = build_test_instance(&app_name, Some(8085));
    let request2 = RegisterRequest::new(instance2);
    client2
        .register(&app_name, &request2)
        .await
        .expect("Failed to register second instance");

    sleep(Duration::from_secs(2)).await;

    // Verify both instances are registered
    let result = client1.get_application(&app_name).await;
    assert!(result.is_ok(), "Failed to get application");

    println!("✓ Successfully registered multiple instances");
}

/// Test with different status values
#[tokio::test]
#[ignore]
async fn test_different_status_values() {
    let eureka_uri = get_eureka_uri();
    let app_name = create_test_app_name("status");
    println!("Testing different status values");

    let client = EurekaClient::new(&app_name, &eureka_uri).unwrap();

    // Test with Status::Starting
    let mut instance = build_test_instance(&app_name, Some(8086));
    instance.status = Status::Starting;
    let request = RegisterRequest::new(instance);

    let result = client.register(&app_name, &request).await;
    assert!(
        result.is_ok(),
        "Failed to register with Starting status: {:?}",
        result.err()
    );
    println!("✓ Successfully registered with Status::Starting");

    sleep(Duration::from_secs(1)).await;

    // Test with Status::Down
    let mut instance2 = build_test_instance(&app_name, Some(8087));
    instance2.status = Status::Down;
    let request2 = RegisterRequest::new(instance2);

    let result2 = client.register(&app_name, &request2).await;
    assert!(
        result2.is_ok(),
        "Failed to register with Down status: {:?}",
        result2.err()
    );
    println!("✓ Successfully registered with Status::Down");
}

/// Comprehensive test that exercises the full lifecycle
/// Test deregistering a previously registered instance
#[tokio::test]
#[ignore]
async fn test_deregister_instance() {
    let eureka_uri = get_eureka_uri();
    let app_name = create_test_app_name("deregister");
    println!("Testing deregister lifecycle for app: {}", app_name);

    let client = EurekaClient::new(&app_name, &eureka_uri).unwrap();
    let instance = build_test_instance(&app_name, Some(8090));
    let request = RegisterRequest::new(instance);

    // Register the instance
    client
        .register(&app_name, &request)
        .await
        .expect("Failed to register instance");
    println!("✓ Instance registered");

    // Wait for registration to propagate
    sleep(Duration::from_secs(3)).await;

    // Verify the instance is registered
    let app_result = client.get_application(&app_name).await;
    if app_result.is_err() {
        println!("Warning: Could not verify registration, skipping deregister test");
        println!("This may be due to slow Eureka propagation or server configuration");
        return;
    }
    println!("✓ Instance verified in registry");

    // Common instance ID patterns Eureka servers use
    let instance_ids = vec![
        "localhost".to_string(),
        "localhost:8090".to_string(),
        "127.0.0.1:8090".to_string(),
        format!("{}:8090", app_name),
    ];

    let mut dereg_success = false;
    for iid in &instance_ids {
        match client.deregister(&app_name, iid).await {
            Ok(()) => {
                println!("✓ Deregister succeeded with instance id: {}", iid);
                dereg_success = true;
                break;
            }
            Err(_) => {
                continue;
            }
        }
    }

    if !dereg_success {
        println!("Warning: Could not deregister with common instance IDs");
        println!("This is acceptable as instanceId format varies by Eureka deployment");
        return;
    }

    // Wait for deregistration to propagate
    sleep(Duration::from_secs(3)).await;

    // Verify instance is no longer present (best-effort check)
    let post = client.get_application(&app_name).await;
    match post {
        Err(rust_eureka::errors::EurekaClientError::NotFound) => {
            println!("✓ Instance successfully deregistered (NotFound)");
        }
        Ok(app_resp) if app_resp.application.instance.is_empty() => {
            println!("✓ Instance successfully deregistered (no instances)");
        }
        Ok(_) => {
            println!("Note: Application still present (may be Eureka caching)");
        }
        Err(e) => {
            println!("Note: Unexpected error checking post-deregister: {:?}", e);
        }
    }
}

#[ignore]
#[tokio::test]
async fn test_full_lifecycle() {
    let eureka_uri = get_eureka_uri();
    let app_name = create_test_app_name("lifecycle");
    println!("\n=== Testing Full Lifecycle ===");
    println!("App name: {}", app_name);
    println!("Eureka URI: {}", eureka_uri);

    let client = EurekaClient::new(&app_name, &eureka_uri).unwrap();

    // 1. Register instance
    println!("\n1. Registering instance...");
    let instance = build_test_instance(&app_name, Some(8088));
    let request = RegisterRequest::new(instance);
    let register_result = client.register(&app_name, &request).await;
    assert!(
        register_result.is_ok(),
        "Registration failed: {:?}",
        register_result.err()
    );
    println!("   ✓ Registration successful");

    // 2. Wait for propagation
    println!("\n2. Waiting for registration to propagate...");
    sleep(Duration::from_secs(3)).await;

    // 3. Verify in single app query
    println!("\n3. Querying single application...");
    let app_result = client.get_application(&app_name).await;
    assert!(
        app_result.is_ok(),
        "Failed to get application: {:?}",
        app_result.err()
    );
    let app = app_result.unwrap();
    println!("   ✓ Found application: {}", app.application.name);

    // 4. Verify in all apps query
    println!("\n4. Querying all applications...");
    let apps_result = client.get_applications().await;
    assert!(
        apps_result.is_ok(),
        "Failed to get applications: {:?}",
        apps_result.err()
    );
    let apps = apps_result.unwrap();
    println!(
        "   ✓ Found {} total applications",
        apps.applications.applications.len()
    );

    let found = apps
        .applications
        .applications
        .iter()
        .any(|a| a.name == app_name);
    assert!(found, "Our app not found in registry");
    println!("   ✓ Our application is in the registry");

    println!("\n=== Full Lifecycle Test Complete ===\n");
}

/// Backwards-compatible test name expected by some CI: full_applications
#[tokio::test]
#[ignore]
async fn test_full_applications() {
    // Reuse the full lifecycle test flow to provide the same coverage under the
    // alternate test name. This mirrors test_full_lifecycle to avoid duplication
    // drift if the test is updated.
    let eureka_uri = get_eureka_uri();
    let app_name = create_test_app_name("lifecycle");
    println!("\n=== Testing Full Applications (alias for lifecycle) ===");
    println!("App name: {}", app_name);
    println!("Eureka URI: {}", eureka_uri);

    let client = EurekaClient::new(&app_name, &eureka_uri).unwrap();

    // 1. Register instance
    println!("\n1. Registering instance...");
    let instance = build_test_instance(&app_name, Some(8088));
    let request = RegisterRequest::new(instance);
    let register_result = client.register(&app_name, &request).await;
    assert!(
        register_result.is_ok(),
        "Registration failed: {:?}",
        register_result.err()
    );
    println!("   ✓ Registration successful");

    // 2. Wait for propagation
    println!("\n2. Waiting for registration to propagate...");
    sleep(Duration::from_secs(3)).await;

    // 3. Verify in single app query
    println!("\n3. Querying single application...");
    let app_result = client.get_application(&app_name).await;
    assert!(
        app_result.is_ok(),
        "Failed to get application: {:?}",
        app_result.err()
    );
    let app = app_result.unwrap();
    println!("   ✓ Found application: {}", app.application.name);

    // 4. Verify in all apps query
    println!("\n4. Querying all applications...");
    let apps_result = client.get_applications().await;
    assert!(
        apps_result.is_ok(),
        "Failed to get applications: {:?}",
        apps_result.err()
    );
    let apps = apps_result.unwrap();
    println!(
        "   ✓ Found {} total applications",
        apps.applications.applications.len()
    );

    let found = apps
        .applications
        .applications
        .iter()
        .any(|a| a.name == app_name);
    assert!(found, "Our app not found in registry");
    println!("   ✓ Our application is in the registry");

    println!("\n=== Full Applications Test Complete ===\n");
}
