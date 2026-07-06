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
cargo test test_name    # Run a single test by name (matches across all modules)
```

### Running
```bash
# Basic usage (connects to production server at connl.io)
cargo run -- 3000

# With custom subdomain
cargo run -- 3000 --subdomain myapp

# Local development (with a local tunnel server, see bindlocal-server)
export HOST_SERVER_TCP=localhost:9090
export HOST_SERVER_HTTP=localhost:8080
cargo run -- 3000
```

### Version Management
When updating the version, change it in TWO places (they must match exactly):
1. `Cargo.toml` - package.version
2. `src/main.rs` - `CLIENT_VERSION` constant

## Architecture

### Core Flow

1. **CLI parsing and orchestration** (`main.rs`): Parses args with clap, handles `--version`, requires a port, then delegates to `connection` and `request_handler`.
2. **Connection Establishment** (`connection.rs`): Connects to the tunnel server via TCP and performs the handshake — sends `"connl {VERSION} {optional_subdomain}"` and reads back either the assigned subdomain or an error. `ERR001` means a client/server version mismatch.
3. **Request/Response Loop** (`request_handler.rs`): Once connected, loops reading one HTTP request at a time from the server stream (headers up to `\r\n\r\n`, then body via Content-Length), forwards it to the local application via `tcp_capture`, writes the response back to the server stream, and prints a status line for each request.
4. **Local Application Communication** (`tcp_capture.rs`): Opens a TCP connection to `localhost:{port}`, forwards the raw request bytes, and reads back the full response — supporting both `Content-Length` and chunked `Transfer-Encoding` bodies. Returns an error if the local app is unreachable (surfaced upstream as `CLIENT_ERROR:ERR_CONNECTION_REFUSED`).

### Module Responsibilities

- **`main.rs`**: CLI parsing (clap), top-level orchestration, holds `CLIENT_VERSION` and `HOST_NAME` constants.
- **`connection.rs`**: Server handshake and subdomain negotiation.
- **`request_handler.rs`**: Main request/response loop — reads from the tunnel server, forwards to the local app, writes the response back, and feeds the terminal display.
- **`tcp_capture.rs`**: Forwards a raw HTTP request to the local application and captures its full response (Content-Length or chunked).
- **`request.rs`**: HTTP header parsing utilities (Content-Length, request line formatting for display).
- **`monitor.rs`**: Prints the initial connection banner (public URL, local port) with colored output.
- **`scrolling_text.rs`**: In-place scrolling terminal log using ANSI escape codes (keeps the last N lines redrawn in place).

### Key Technical Details

**HTTP Parsing**:
- Header/body delimiter: `\r\n\r\n`
- Chunked encoding terminator: `0\r\n\r\n`
- Read buffer size: 4096 bytes

**Error Handling**:
- `CLIENT_ERROR:ERR_CONNECTION_REFUSED`: Local application unavailable on the given port.
- `ERR001`: Client version mismatch — server requires an updated client.

**Environment Variables**:
- `HOST_SERVER_TCP`: Override tunnel server TCP address (default: `connl.io:9090`).
- `HOST_SERVER_HTTP`: Override tunnel server HTTP hostname shown in the banner (default: `connl.io`).

## Testing Strategy

Tests are embedded in each module using `#[cfg(test)]`, next to the code they cover:
- `request.rs`: HTTP header parsing (Content-Length, request line format).
- `tcp_capture.rs`: HTTP response status-line parsing.

When adding new HTTP parsing logic, add corresponding unit tests in the same file.

## Related Repositories

This is the **client** component. The server component is at: https://github.com/khajer/bindlocal-server

For local development, both client and server must be running:
1. Start server: `cd ../bindlocal-server && cargo run`
2. Start client: `export HOST_SERVER_TCP=localhost:9090 && cargo run -- 3000`
