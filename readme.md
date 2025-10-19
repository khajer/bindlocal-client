# Connl Application
expose your local app to the internet with one command line. the host is connl.io

# build from source
```sh
cargo build --release
```

# how to install with brew
```sh
brew tap khajer/connl
brew install connl
```

# how to use
```sh
connl 3000 # expose localhost:3000 to connl.io
```

# development on localhost
you can develop this programming with own server

## server
pull the code at [https://github.com/khajer/bindlocal-server](https://github.com/khajer/bindlocal-server)
```sh
git clone git@github.com:khajer/bindlocal-server.git
cd bindlocal-server
cargo run
```

## client
set environment localhost server before run
```sh
export HOST_SERVER_TCP=localhost:9090
export HOST_SERVER_HTTP=localhost:8080
```
