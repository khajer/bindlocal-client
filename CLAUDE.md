# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Connl is a local tunneling service client written in Rust that creates secure tunnels from the internet to local development servers. The client establishes a persistent TCP connection to a remote server (connl.io), which forwards HTTP/HTTPS requests through the tunnel to the local application.

## Development Commands

### Building
```bash
cargo build              # Debug build
cargo build --release    # Production build (outputs to target/release/connl)
```

### Testing
```bash
cargo test              # Run all tests
cargo test --verbose    # Run tests with detailed output
```

### Running
```bash
# Basic usage (connects to production server)
cargo run -- 3000

# With custom subdomain
cargo run -- 3000 --subdomain myapp

# Local development (with local tunnel server)
export HOST_SERVER_TCP=localhost:9090
export HOST_SERVER_HTTP=localhost:8080
cargo run -- 3000
```

### Version Management
When updating the version, change it in TWO places:
1. `Cargo.toml` - package.version
2. `src/main.rs` - CLIENT_VERSION constant

## Architecture

### Core Flow
1. **Connection Establishment** (`main.rs` lines 54-117):
   - Client connects to tunnel server via TCP
   - Sends handshake: `"connl {VERSION} {optional_subdomain}"`
   - Server responds with assigned subdomain or error
   - Error ERR_001 indicates version mismatch

2. **Request/Response Loop** (`main.rs` lines 125-205):
   - Client reads HTTP request from server (headers + body)
   - Parses Content-Length to determine body size
   - Forwards complete request to local application
   - Captures response and sends back to server
   - Displays request summary in terminal

3. **Local Application Communication** (`tcp_capture.rs`):
   - Connects to localhost on specified port
   - Supports both Content-Length and chunked Transfer-Encoding responses
   - Handles complete request/response lifecycle
   - Returns error if local application is unreachable

### Module Responsibilities

- **`main.rs`**: CLI parsing (clap), server connection, request/response orchestration
- **`tcp_capture.rs`**: HTTP response capture from local application (handles both Content-Length and chunked encoding)
- **`request.rs`**: HTTP header parsing utilities (Content-Length, request format)
- **`monitor.rs`**: Terminal UI for displaying connection status with colors
- **`scrolling_text.rs`**: Scrolling log display (maintains last 4 lines, uses ANSI escape codes)

### Key Technical Details

**HTTP Parsing**:
- Delimiter for HTTP headers: `\r\n\r\n` (TWO_DELIMETER_BYTES)
- Chunked encoding terminator: `0\r\n\r\n` (END_DELIMETER_BYTES)
- Buffer size: 4096 bytes

**Error Handling**:
- `CLIENT_ERROR:ERR_CONNECTION_REFUSED`: Local application unavailable
- `ERR001`: Client version mismatch (server requires update)

**Environment Variables**:
- `HOST_SERVER_TCP`: Override tunnel server TCP address (default: connl.io:9090)
- `HOST_SERVER_HTTP`: Override tunnel server HTTP address (default: connl.io)

## Testing Strategy

Tests are embedded in each module using `#[cfg(test)]`:
- `request.rs`: HTTP header parsing (Content-Length, request format)
- `tcp_capture.rs`: HTTP response header parsing

When adding new HTTP parsing logic, add corresponding unit tests in the same file.

## Related Repositories

This is the **client** component. The server component is at: https://github.com/khajer/bindlocal-server

For local development, both client and server must be running:
1. Start server: `cd ../bindlocal-server && cargo run`
2. Start client: `export HOST_SERVER_TCP=localhost:9090 && cargo run -- 3000`
