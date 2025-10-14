use std::env;
use std::error::Error;
use std::str;
// use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
// use uuid::Uuid;

mod request;
mod tcp_capture;
use chrono::{Datelike, Local, Timelike};
use tcp_capture::TcpCapture;

use crate::request::HttpRequest;

#[tokio::main]
async fn main() -> io::Result<()> {
    let host_server;
    if env::var("HOST_SERVER").is_ok() {
        host_server = env::var("HOST_SERVER").unwrap();
    } else {
        host_server = "connl.io:9090".to_string();
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "missing parameter",
        ));
    }
    let local_port = args[1].as_str();

    // Connect to server
    let mut stream = TcpStream::connect(host_server).await?;
    let mut buffer = [0; 1024];

    let n = stream.read(&mut buffer).await?;
    if n == 0 {
        println!("Server Closed Connection.");
    }
    let rec_msg = String::from_utf8_lossy(&buffer[..n]);
    println!("{}", rec_msg);

    loop {
        let mut total_data = Vec::new();
        loop {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                println!("Server Closed Connection.");
                break;
            }
            total_data.extend_from_slice(&buffer[..n]);

            if total_data.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
        }
        let headers_end = total_data
            .windows(4)
            .position(|w| w == b"\r\n\r\n")
            .unwrap()
            + 4;

        let headers_str = str::from_utf8(&total_data[..headers_end - 4]);
        let content_length =
            HttpRequest::parse_content_length(headers_str.expect("NOT_FOUND_CONTENT_LENGTH"));

        if let Some(body_length) = content_length {
            let body_data_received = total_data.len() - headers_end;
            let remaining_body = body_length - body_data_received;
            if remaining_body > 0 {
                let mut body_buf = vec![0u8; remaining_body];
                let mut bytes_read = 0;

                while bytes_read < remaining_body {
                    let n = stream.read(&mut body_buf[bytes_read..]).await?;
                    if n == 0 {
                        println!("Server Closed Connection.");
                    }
                    bytes_read += n;
                }

                total_data.extend_from_slice(&body_buf);
            }
        }
        // let trx_id = Uuid::new_v4();

        // save_log_req_resp(format!("[{trx_id}] request").as_str(), &total_data).await;
        let host = format!("localhost:{local_port}");
        let response_data = TcpCapture::capture_http_raw(&total_data, host.as_str())
            .await
            .unwrap();

        if let Err(e) = stream.write_all(&response_data).await {
            println!("Send to server fails {:?}", e);
            break;
        }
        // save_log_req_resp(format!("[{trx_id}] response").as_str(), &response_data).await;

        if let Err(e) = stream.flush().await {
            eprintln!("Error flushing TCP stream: {}", e);
            break;
        }
    }

    Ok(())
}
