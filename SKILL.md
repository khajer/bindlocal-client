# SKILL.md

This file defines project-specific skills for the Connl tunneling client.

## Project Skills

### build
**Description**: Build the Connl client
**Usage**: Build the project in debug or release mode
**Commands**:
```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### test
**Description**: Run the test suite
**Usage**: Execute all unit tests in the project
**Commands**:
```bash
# Run all tests
cargo test

# Run with verbose output
cargo test --verbose
```

### run-local
**Description**: Run client connecting to local tunnel server
**Usage**: Start the client for local development with a local server
**Environment Variables**:
- HOST_SERVER_TCP=localhost:9090
- HOST_SERVER_HTTP=localhost:8080
**Commands**:
```bash
# Basic local run
HOST_SERVER_TCP=localhost:9090 HOST_SERVER_HTTP=localhost:8080 cargo run -- 3000

# With custom subdomain
HOST_SERVER_TCP=localhost:9090 HOST_SERVER_HTTP=localhost:8080 cargo run -- 3000 --subdomain myapp
```

### run-production
**Description**: Run client connecting to production server
**Usage**: Start the client for production testing
**Commands**:
```bash
# Connect to production
cargo run -- 3000

# With custom subdomain
cargo run -- 3000 --subdomain myapp
```

### update-version
**Description**: Update the client version
**Usage**: Updates version in both required locations
**Important**: Version MUST be updated in TWO places:
1. `Cargo.toml` - package.version (line 3)
2. `src/main.rs` - CLIENT_VERSION constant (line 12)
**Process**:
1. Edit Cargo.toml and update version field
2. Edit src/main.rs and update CLIENT_VERSION constant
3. Ensure both versions match exactly
4. Run tests to verify: `cargo test`
5. Test version display: `cargo run -- --version`

### check-modules
**Description**: Verify module structure and dependencies
**Usage**: Check that all modules are properly organized
**Modules**:
- `main.rs`: CLI parsing, server connection, orchestration
- `connection.rs`: Server connection and handshake
- `request_handler.rs`: Request/response loop orchestration
- `request.rs`: HTTP header parsing utilities
- `tcp_capture.rs`: HTTP response capture from local app
- `monitor.rs`: Terminal UI for connection status
- `scrolling_text.rs`: Scrolling log display

### local-dev-setup
**Description**: Setup local development environment
**Usage**: Prepare for local development with server
**Prerequisites**:
- Server repo must be cloned at ../bindlocal-server
**Commands**:
```bash
# Terminal 1: Start server
cd ../bindlocal-server
cargo run

# Terminal 2: Start client
cd bindlocal-client
export HOST_SERVER_TCP=localhost:9090
export HOST_SERVER_HTTP=localhost:8080
cargo run -- 3000
```

### release-build
**Description**: Create optimized release binary
**Usage**: Build production-ready binary
**Commands**:
```bash
# Build release binary
cargo build --release

# Binary location
ls -lh target/release/connl

# Test release binary
./target/release/connl --version
./target/release/connl 3000
```

### add-test
**Description**: Add a new test to a module
**Usage**: Create unit tests following project conventions
**Guidelines**:
- Tests are embedded in each module using `#[cfg(test)]`
- Place tests at the end of the file
- Test HTTP parsing logic in `request.rs` and `tcp_capture.rs`
- Example pattern:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Test implementation
    }
}
```

### troubleshoot
**Description**: Common troubleshooting steps
**Usage**: Debug common issues
**Common Issues**:
1. **ERR_CONNECTION_REFUSED**: Local application not running on specified port
   - Solution: Start local app on the port you're tunneling
2. **ERR001**: Client version mismatch with server
   - Solution: Update client version
3. **Connection timeout**: Server unreachable
   - Solution: Check network or use local development setup

## Skill Chaining Examples

### Full Development Cycle
```bash
# 1. Build and test
build && test

# 2. Run locally
local-dev-setup

# 3. Test production
run-production
```

### Release Process
```bash
# 1. Update version
update-version

# 2. Run tests
test

# 3. Create release build
release-build

# 4. Test binary
./target/release/connl --version
```

## Notes

- All skills respect the project structure defined in CLAUDE.md
- Environment variables can override default server addresses
- Tests should be run before any release
- Version updates require changes in TWO locations (never forget!)
