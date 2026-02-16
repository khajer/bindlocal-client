use std::env;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const CLIENT_VERSION: &str = "0.1.1";
const HOST_SERVER_TCP: &str = "connl.io:9090";
const ERR_001: &str = "ERR001";
const TXT_ERR: &str = "err";

pub struct Connection;

impl Connection {
    /// Establishes a connection to the tunnel server and performs handshake
    /// Returns the TcpStream and the subdomain assigned by the server
    pub async fn connect_to_server(
        subdomain: Option<String>,
    ) -> io::Result<(TcpStream, String)> {
        let host_server = env::var("HOST_SERVER_TCP")
            .unwrap_or_else(|_| HOST_SERVER_TCP.to_string());

        // Connect to server
        let mut stream = TcpStream::connect(host_server).await?;
        let mut buffer = [0; 4096];

        // Send handshake message
        let req_connect = if let Some(subdomain) = subdomain {
            format!("connl {CLIENT_VERSION} {subdomain}")
        } else {
            format!("connl {CLIENT_VERSION}")
        };

        stream.write_all(req_connect.as_bytes()).await.map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Send data to server fails: {:?}", e))
        })?;

        stream.flush().await.map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Error flushing TCP stream: {}", e))
        })?;

        // Read server response
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            return Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "Server Closed Connection.",
            ));
        }

        let rec_msg = String::from_utf8_lossy(&buffer[..n]);

        // Handle errors from server
        if rec_msg.to_string().to_lowercase().contains(TXT_ERR) {
            let err_code = rec_msg.split(':').nth(0).unwrap_or("Unknown");
            let error_msg = if err_code == ERR_001 {
                "please update version: https://connl.io/update_version.html"
            } else {
                &format!("Connect Server Error: {rec_msg}")
            };
            return Err(io::Error::new(io::ErrorKind::Other, error_msg));
        }

        Ok((stream, rec_msg.to_string()))
    }
}
