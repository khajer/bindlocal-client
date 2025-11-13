# Connl - Local Tunneling Service

Expose your local applications to the internet with a single command. Connl creates a secure tunnel from the internet to your local development server, making it easy to share your work with others or test webhooks.

## Features

- **Instant Public URLs**: Get a public URL for your local application in seconds
- **Custom Subdomains**: Use custom subdomains for your tunnels
- **HTTPS Support**: Automatic HTTPS with SSL certificates
- **HTTP/HTTPS Tunneling**: Works with both HTTP and HTTPS local servers
- **Real-time Request Monitoring**: View incoming requests in real-time with colored output
- **Cross-platform**: Works on macOS, Linux, and Windows

## Installation

### Curl  (Recommended)
```sh
curl -sSfL https://raw.githubusercontent.com/khajer/bindlocal-client/main/install.sh | bash
```

### Build from Source
```sh
git clone https://github.com/khajer/bindlocal-client.git
cd bindlocal-client
cargo build --release
```

The binary will be available at `target/release/connl`.

## Quick Start
Expose your local application running on port 3000:

```sh
connl 3000
```

This will give you a public URL like:
- `http://app-1290.connl.io` → `http://localhost:3000`
- `https://app-1290.connl.io` → `http://localhost:3000`

The server will give a random subdomain. (ex. app-1290)

## 📖 Usage

### Basic Usage

```sh
# Expose localhost:3000
connl 3000

```

### Custom Subdomain

```sh
# Use a custom subdomain
connl 3000 --subdomain myapp

# This will create: myapp.connl.io
```

### Development with Local Server

For local development, you'll need both the client and server components:
1. **Set up the server**:
   ```sh
   git clone https://github.com/khajer/bindlocal-server.git
   cd bindlocal-server
   cargo run
   ```

2. **Set up the client**:
   ```sh
   # In another terminal
   export HOST_SERVER_TCP=localhost:9090
   export HOST_SERVER_HTTP=localhost:8080
   cargo run
   ```

## Requirements

- Rust 1.70+ (for building from source)
- Network connectivity to connl.io servers
- Local application running on the specified port

## Architecture

Connl consists of two main components:

1. **Client (this repository)**: Runs on your local machine and forwards requests
2. **Server** [(server repository)](https://github.com/khajer/bindlocal-server): Manages tunnels and routes external traffic to your client

The client establishes a persistent TCP connection to the server, which then forwards HTTP/HTTPS requests through this tunnel to your local application.


## Link

- [Server Repository](https://github.com/khajer/bindlocal-server)
- [Client Repository](https://github.com/khajer/bindlocal-client)
- [Website](https://connl.io)

## Contact Me
- [khajer@gmail.com](mailto:khajer@gmail.com)
