# rust_eureka

<!-- markdownlint-disable line-length --->

[![Crates.io](https://img.shields.io/crates/v/rust-eureka.svg)](https://crates.io/crates/rust-eureka)
[![Build Status](https://img.shields.io/github/actions/workflow/status/ayax79/rust_eureka/ci.yml?branch=main)](https://github.com/ayax79/rust_eureka/actions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A Rust implementation of a client for Netflix
[Eureka](https://github.com/Netflix/eureka)

## Features

- Service registration
- Application discovery (single and all)
- Async/await with Tokio
- Full JSON serialization/deserialization
- Type-safe API
- Comprehensive error handling

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rust-eureka = "0.2"
```

## Usage

```rust
use rust_eureka::{EurekaClient, request::{Instance, RegisterRequest, Status, DataCenterInfo, DcName}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Eureka client
    let client = EurekaClient::new("my-service", "http://localhost:8080");

    // Build an instance registration
    let instance = Instance {
        host_name: "localhost".to_owned(),
        app: "MY_SERVICE".to_owned(),
        ip_addr: "127.0.0.1".to_owned(),
        vip_address: "127.0.0.1".to_owned(),
        secure_vip_address: "127.0.0.1".to_owned(),
        status: Status::Up,
        port: Some(8080),
        secure_port: None,
        homepage_url: "http://localhost:8080".to_owned(),
        status_page_url: "http://localhost:8080/status".to_owned(),
        health_check_url: "http://localhost:8080/health".to_owned(),
        data_center_info: DataCenterInfo {
            name: DcName::MyOwn,
            metadata: None,
        },
        lease_info: None,
        metadata: serde_json::Map::new(),
    };

    let request = RegisterRequest::new(instance);

    // Register with Eureka
    client.register("MY_SERVICE", &request).await?;
    println!("Successfully registered with Eureka");

    // Query for a specific application
    let app = client.get_application("MY_SERVICE").await?;
    println!("Found application: {}", app.application.name);

    // Query all applications
    let apps = client.get_applications().await?;
    println!("Total applications: {}", apps.applications.applications.len());

    Ok(())
}
```

## Testing

### Unit Tests

Run the unit tests:

```bash
cargo test
```

### Integration Tests

The project includes comprehensive integration tests that run against a real
Eureka server. See [INTEGRATION_TESTING.md](INTEGRATION_TESTING.md) for detailed
instructions.

Quick start:

```bash
# Start Eureka with Docker
docker run -p 8080:8080 springcloud/eureka

# Run integration tests
EUREKA_URI=http://localhost:8080 cargo test --test integration_tests -- --ignored --test-threads=1
```

## Development

This project follows strict Rust quality standards:

CI: GitHub Actions workflow added to run formatting, clippy, tests, docs, and
security/coverage checks. See .github/workflows/ci.yml

- Zero clippy warnings with `-D warnings`
- Formatted with `rustfmt`
- 80%+ code coverage
- Comprehensive documentation

See [AGENT_GUIDELINES.md](.project_hints) for detailed development guidelines.

### Pre-commit Checklist

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps
```

## API Endpoints

This client implements the Eureka v2 REST API:

- `POST /v2/apps/{appID}` - Register instance
- `GET /v2/apps/{appID}` - Get application
- `GET /v2/apps` - Get all applications

For more details, see the
[Eureka REST API documentation](https://github.com/Netflix/eureka/wiki/Eureka-REST-operations).

## License

Licensed under the MIT license. See [LICENSE](LICENSE) for details.
