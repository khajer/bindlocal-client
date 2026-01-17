# Docker for Connl

This guide covers how to build, run, and manage Connl (Local Tunneling Service Client) using Docker and Docker Compose.

## Table of Contents

- [Quick Start](#quick-start)
- [Building the Docker Image](#building-the-docker-image)
- [Running with Docker](#running-with-docker)
- [Using Docker Compose](#using-docker-compose)
- [Configuration](#configuration)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)
- [Advanced Usage](#advanced-usage)
- [Security Considerations](#security-considerations)
- [Performance Optimization](#performance-optimization)

## Quick Start

### Using Docker Compose

```bash
# Build and run with default settings (exposes localhost:3000)
docker-compose up

# Expose a different port
docker-compose run connl 8080

# Expose with custom subdomain
docker-compose run connl 3000 --subdomain myapp
```

### Using Docker directly

```bash
# Build the image
docker build -t connl:latest .

# Run to expose localhost:3000
docker run --network host connl:latest 3000

# Run with custom subdomain
docker run --network host connl:latest 3000 --subdomain myapp
```

## Building the Docker Image

The Dockerfile uses a multi-stage build process for optimal image size:

### Build manually

```bash
# Build with latest tag
docker build -t connl:latest .

# Build with specific version tag
docker build -t connl:0.1.1 .

# Build without cache
docker build --no-cache -t connl:latest .
```

### Build with build arguments

```bash
# Build with custom Rust version
docker build --build-arg RUST_VERSION=1.82 -t connl:latest .

# Build with custom Alpine version
docker build --build-arg ALPINE_VERSION=3.19 -t connl:latest .
```

### Build output

The final image is based on Alpine Linux and includes:
- The `connl` binary in `/usr/local/bin/connl`
- CA certificates for SSL/TLS
- A non-root user named `connl` (uid: 1000)

Image size: Approximately 8-12 MB

## Running with Docker

### Basic Usage

```bash
# Expose localhost:3000
docker run --rm --network host connl:latest 3000

# Expose with custom subdomain
docker run --rm --network host connl:latest 3000 --subdomain myapp

# Show help
docker run --rm connl:latest --help
```

### Network Modes

**Host Network (Recommended for local development)**

```bash
docker run --rm --network host connl:latest 3000
```

Allows the container to access services running on your host machine's `localhost`.

**Bridge Network**

```bash
docker run --rm -p 8080:8080 connl:latest 8080
```

Use this if the application you want to expose is also running in a container.

### Environment Variables

```bash
# Set custom server endpoints
docker run --rm --network host \
  -e HOST_SERVER_TCP=your-server.com:9090 \
  -e HOST_SERVER_HTTP=your-server.com:8080 \
  connl:latest 3000

# Enable debug logging
docker run --rm --network host \
  -e RUST_LOG=debug \
  connl:latest 3000
```

### Volume Mounts

```bash
# Mount a directory for logs (if implemented in future)
docker run --rm --network host \
  -v /path/to/logs:/home/connl/logs \
  connl:latest 3000
```

### Detached Mode

```bash
# Run in background
docker run -d --name connl --network host connl:latest 3000

# View logs
docker logs -f connl

# Stop the container
docker stop connl

# Remove the container
docker rm connl
```

## Using Docker Compose

### Standard Usage

The `docker-compose.yml` file is configured for production use with the official Connl servers.

```bash
# Start the service
docker-compose up

# Start in detached mode
docker-compose up -d

# View logs
docker-compose logs -f

# Stop the service
docker-compose down

# Restart the service
docker-compose restart
```

### Custom Ports

Edit the `docker-compose.yml` file:

```yaml
services:
  connl:
    command: ["8080"]  # Change this to your desired port
```

Or override on the command line:

```bash
docker-compose run connl 8080
```

### Custom Subdomain

```bash
docker-compose run connl 3000 --subdomain myapp
```

### Development Usage

For local development with a local Connl server, use the development configuration:

```bash
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up
```

The `docker-compose.dev.yml` file sets:
- `HOST_SERVER_TCP=localhost:9090`
- `HOST_SERVER_HTTP=localhost:8080`

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `HOST_SERVER_TCP` | TCP server address | `connl.io:9090` |
| `HOST_SERVER_HTTP` | HTTP server address | `connl.io:8080` |
| `RUST_LOG` | Logging level (error, warn, info, debug, trace) | `info` |

### Docker Compose Override Files

Create `docker-compose.override.yml` to customize settings without modifying the main files:

```yaml
version: '3.8'

services:
  connl:
    environment:
      - HOST_SERVER_TCP=custom-server.com:9090
      - RUST_LOG=debug
    command: ["8080"]
```

## Examples

### Example 1: Expose a Node.js Application

```bash
# Start your Node.js app on port 3000
node server.js &

# Expose it with Connl
docker run --rm --network host connl:latest 3000 --subdomain my-node-app
```

### Example 2: Expose a Python Flask App

```bash
# Start Flask app on port 5000
python app.py &

# Expose it with Connl
docker run --rm --network host connl:latest 5000 --subdomain flask-demo
```

### Example 3: Multiple Tunnels

```bash
# Start multiple tunnels for different apps
docker run -d --name tunnel1 --network host connl:latest 3000 --subdomain app1
docker run -d --name tunnel2 --network host connl:latest 4000 --subdomain app2
docker run -d --name tunnel3 --network host connl:latest 5000 --subdomain app3

# View all tunnels
docker ps --filter name=tunnel
```

### Example 4: Local Development Setup

```bash
# Terminal 1: Start the local Connl server
cd ../bindlocal-server
cargo run

# Terminal 2: Start the client using Docker Compose dev mode
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up
```

### Example 5: Persistent Logging

```bash
# Run with log output to file
docker run --rm --network host \
  -v $(pwd)/logs:/home/connl/logs \
  connl:latest 3000 > connl.log 2>&1

# Or use Docker's logging
docker run -d --name connl \
  --network host \
  --log-driver json-file \
  --log-opt max-size=10m \
  --log-opt max-file=3 \
  connl:latest 3000
```

## Troubleshooting

### Container won't start

```bash
# Check the logs
docker logs <container-id>

# Run with debug logging
docker run --rm --network host -e RUST_LOG=debug connl:latest 3000
```

### Cannot connect to localhost

Make sure you're using `--network host` or your local application is accessible from the container.

```bash
# Test connectivity
docker run --rm --network host alpine ping -c 3 localhost

# Check if your app is running
curl http://localhost:3000
```

### Connection refused

- Verify the local application is running on the specified port
- Check firewall settings
- Ensure the Connl server is accessible

```bash
# Check if port is in use
lsof -i :3000  # macOS/Linux
netstat -an | grep 3000  # Linux

# Test connection to Connl server
curl https://connl.io
```

### Permission issues

The Docker container runs as a non-root user (uid: 1000). If you need to access files with specific permissions:

```bash
# Run with specific user ID
docker run --rm --network host -u $(id -u):$(id -g) connl:latest 3000
```

### Image build failures

```bash
# Clean build with no cache
docker build --no-cache -t connl:latest .

# Check Docker version
docker --version

# Verify disk space
df -h
```

## Advanced Usage

### Custom Dockerfile

Create a custom `Dockerfile.custom`:

```dockerfile
FROM rust:1.83-alpine AS builder

RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY src ./src

# Add custom build steps here
RUN cargo build --release --features debug

FROM alpine:3.20

RUN apk add --no-cache ca-certificates

RUN addgroup -g 1000 connl && \
    adduser -D -u 1000 -G connl connl

WORKDIR /home/connl

COPY --from=builder /app/target/release/connl /usr/local/bin/connl

RUN chown -R connl:connl /home/connl

USER connl

ENTRYPOINT ["connl"]
CMD ["--help"]
```

Build and run:

```bash
docker build -f Dockerfile.custom -t connl:custom .
docker run --rm --network host connl:custom 3000
```

### Docker Swarm

```bash
# Deploy to Swarm
docker stack deploy -c docker-compose.yml connl

# Scale the service
docker service scale connl_connl=3

# View services
docker service ls
```

### Kubernetes

Create `k8s-deployment.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: connl-client
spec:
  replicas: 1
  selector:
    matchLabels:
      app: connl
  template:
    metadata:
      labels:
        app: connl
    spec:
      containers:
      - name: connl
        image: connl:latest
        args: ["3000"]
        env:
        - name: HOST_SERVER_TCP
          value: "connl.io:9090"
        - name: HOST_SERVER_HTTP
          value: "connl.io:8080"
        hostNetwork: true
```

Deploy:

```bash
kubectl apply -f k8s-deployment.yaml
kubectl logs -f deployment/connl-client
```

### Health Checks

Modify `docker-compose.yml`:

```yaml
services:
  connl:
    build: .
    image: connl:latest
    container_name: connl-client
    network_mode: host
    healthcheck:
      test: ["CMD", "nc", "-z", "localhost", "3000"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 10s
```

## Security Considerations

1. **Non-root user**: The container runs as a non-root user by default
2. **Read-only filesystem**: Consider adding `read_only: true` in Docker Compose for extra security
3. **Secrets management**: Use Docker secrets or environment files for sensitive data

```bash
# Use environment file
echo "HOST_SERVER_TCP=custom-server.com:9090" > .env
docker-compose --env-file .env up
```

## Performance Optimization

### Build caching

The Dockerfile is optimized for layer caching. Dependencies are built before source code to maximize cache hits.

### Image size

The multi-stage build produces a minimal image (~8-12 MB) by:
- Using Alpine Linux as the base
- Removing build artifacts
- Stripping symbols from the binary

### Resource limits

Set resource limits in `docker-compose.yml`:

```yaml
services:
  connl:
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
        reservations:
          cpus: '0.25'
          memory: 256M
```

## Getting Help

If you encounter issues:

1. Check the logs: `docker logs <container-id>`
2. Enable debug logging: `-e RUST_LOG=debug`
3. Verify network connectivity
4. Consult the main [README.md](readme.md) for general usage
5. Open an issue on GitHub: https://github.com/khajer/bindlocal-client/issues

## License

This Docker configuration follows the same license as the Connl project.
