# Integration Tests for rust_eureka

This document explains how to run integration tests against a real Eureka server.

## Prerequisites

You need a running Eureka server on `http://localhost:8080`. There are several ways to start one:

### Option 1: Using Docker (Recommended)

The easiest way is to use the official Spring Cloud Eureka Docker image:

```bash
docker run -p 8080:8080 springcloud/eureka
```

Wait a few seconds for the server to start. You can verify it's running by visiting http://localhost:8080 in your browser.

### Option 2: Using the Nike Edge Eureka Server

If you have access to the Nike Edge Eureka server source:

```bash
cd ~/nikedev/nike-edge/eureka/eureka-server

# Build the WAR file
gradle war

# Run using Gradle's gretty plugin
gradle appRun

# Or extract and run the WAR with Tomcat/Jetty
```

### Option 3: Download Netflix Eureka

Download and run the official Netflix Eureka server:

```bash
# Clone the Netflix Eureka repository
git clone https://github.com/Netflix/eureka.git
cd eureka/eureka-server

# Build and run
./gradlew build
./gradlew bootRun
```

## Running Integration Tests

Once your Eureka server is running:

```bash
# Run all integration tests (they are ignored by default)
EUREKA_URI=http://localhost:8080 cargo test --test integration_tests -- --ignored --test-threads=1

# Run a specific test
EUREKA_URI=http://localhost:8080 cargo test --test integration_tests test_register_instance -- --ignored

# Run with verbose output
EUREKA_URI=http://localhost:8080 cargo test --test integration_tests -- --ignored --nocapture --test-threads=1
```

**Important:** The `--test-threads=1` flag runs tests sequentially to avoid conflicts between test instances.

## Test Coverage

The integration tests cover the following scenarios:

1. **Connectivity Test** (`test_eureka_connectivity`)
   - Verifies basic connectivity to the Eureka server
   - Checks that the API is accessible

2. **Instance Registration** (`test_register_instance`)
   - Registers a new service instance
   - Verifies the instance appears in the registry

3. **Get Single Application** (`test_get_application`)
   - Retrieves a specific application by name
   - Verifies the application details

4. **Get All Applications** (`test_get_all_applications`)
   - Retrieves all registered applications
   - Verifies our test application is in the list

5. **Error Handling** (`test_get_nonexistent_application`)
   - Tests NotFound error for non-existent applications
   - Verifies proper error handling

6. **Multiple Instances** (`test_register_multiple_instances`)
   - Registers multiple instances of the same application
   - Verifies both instances are tracked

7. **Status Values** (`test_different_status_values`)
   - Tests different instance statuses (Up, Down, Starting)
   - Verifies status handling

8. **Full Lifecycle** (`test_full_lifecycle`)
   - Comprehensive end-to-end test
   - Covers registration, querying, and verification

## Test Application Naming

Each test uses a unique application name to avoid conflicts:
- Format: `RUST_EUREKA_TEST_<TEST_NAME>`
- Example: `RUST_EUREKA_TEST_REGISTER`

This ensures tests can run independently without interfering with each other.

## Troubleshooting

### Connection Refused
```
Error: ClientError(hyper::Error(Connect, ConnectError...))
```
**Solution:** Make sure Eureka server is running on localhost:8080

### 404 Not Found
```
Error: NotFound
```
**Possible causes:**
1. Wrong base URL - ensure you're using `http://localhost:8080` (not `/eureka` or `/v2/apps`)
2. Server not fully started - wait a few more seconds
3. Wrong API version - this client uses Eureka v2 API

### Tests Hanging
**Solution:** Run with `--test-threads=1` to avoid race conditions

### Registration Fails
Check the Eureka server logs for errors. Common issues:
- Invalid JSON in request body
- Required fields missing
- Server-side validation failures

## Cleaning Up

After running tests, registered test instances will remain in Eureka for up to 90 seconds (the eviction duration). They will be automatically removed when their leases expire.

To clean up immediately, restart the Eureka server:
```bash
# If using Docker:
docker ps  # Find the container ID
docker restart <container-id>
```

## CI/CD Integration

For CI/CD pipelines, use Docker Compose to start Eureka:

```yaml
# docker-compose.yml
version: '3'
services:
  eureka:
    image: springcloud/eureka
    ports:
      - "8080:8080"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080"]
      interval: 10s
      timeout: 5s
      retries: 5
```

Then in your CI script:
```bash
# Start Eureka
docker-compose up -d
docker-compose exec -T eureka sh -c 'while ! curl -f http://localhost:8080; do sleep 1; done'

# Run tests
EUREKA_URI=http://localhost:8080 cargo test --test integration_tests -- --ignored --test-threads=1

# Cleanup
docker-compose down
```

## Additional Resources

- [Eureka REST Operations](https://github.com/Netflix/eureka/wiki/Eureka-REST-operations)
- [Eureka Architecture](https://github.com/Netflix/eureka/wiki/Eureka-at-a-glance)
- [Spring Cloud Netflix Eureka](https://cloud.spring.io/spring-cloud-netflix/reference/html/)
