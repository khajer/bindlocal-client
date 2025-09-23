use std::collections::HashMap;

use std::env;
use std::error::Error;
use std::str;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> io::Result<()> {
    if env::var("LOCAL_DEV").is_ok() {
        let local_dev = env::var("LOCAL_DEV").unwrap();
        if local_dev.to_lowercase() == "true".to_string() {
            call_direct().await;
            return Ok(());
        }
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
    let mut stream = TcpStream::connect("127.0.0.1:9090").await?;
    let mut first_message = true;
    let mut buffer = [0; 1024];
    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            println!("Server Closed Connection.");
            break;
        }

        if first_message {
            let rec_msg = String::from_utf8_lossy(&buffer[..n]);
            first_message = !first_message;
            println!("{}", rec_msg);
            continue;
        }

        println!("{:?}", buffer);
        let rec_msg = String::from_utf8_lossy(&buffer[..n]).to_string();
        println!("request : \n{rec_msg}");

        let host = format!("localhost:{local_port}");

        let request_buff = trim_null_bytes(&buffer);

        let response_data = capture_http_raw(&request_buff, host.as_str())
            .await
            .unwrap();

        if let Err(e) = stream.write_all(&response_data).await {
            println!("Send to server fails {:?}", e);
        }
        println!("send completed.");

        if let Err(e) = stream.flush().await {
            eprintln!("Error flushing TCP stream: {}", e);
        }
    }

    Ok(())
}

fn trim_null_bytes(data: &[u8]) -> &[u8] {
    let start = data.iter().position(|&b| b != 0).unwrap_or(data.len());
    let end = data.iter().rposition(|&b| b != 0).unwrap_or(data.len());
    if start >= data.len() {
        &data[0..0]
    } else {
        &data[start..=end]
    }
}

async fn capture_http_raw(request: &[u8], host: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    // let mut stream = TcpStream::connect((host, 3000)).await?;
    println!("Connecting to {}", host);
    match TcpStream::connect(host).await {
        Ok(mut stream) => {
            stream.write_all(request).await?;
            stream.flush().await?;
            let mut buffer = Vec::new();
            let mut tmp = [0u8; 1024];
            let header_end;

            let time = Instant::now();
            loop {
                let n = stream.read(&mut tmp).await?;
                if n == 0 {
                    return Err("connection closed before headers".into());
                }
                buffer.extend_from_slice(&tmp[..n]);
                if let Some(pos) = buffer.windows(4).position(|w| w == b"\r\n\r\n") {
                    header_end = pos + 4;
                    break;
                }
            }

            // --- Parse headers (just enough to know how much to read) ---
            let header_text = String::from_utf8_lossy(&buffer[..header_end]);
            let mut headers = HashMap::new();
            for line in header_text.lines().skip(1) {
                if let Some((k, v)) = line.split_once(": ") {
                    headers.insert(k.to_string(), v.to_string());
                }
            }
            // --- Parse headers (just enough to know how much to read) ---
            let header_text = String::from_utf8_lossy(&buffer[..header_end]);
            let mut headers = HashMap::new();
            for line in header_text.lines().skip(1) {
                if let Some((k, v)) = line.split_once(": ") {
                    headers.insert(k.to_string(), v.to_string());
                }
            }

            // --- Read the body depending on headers ---
            if let Some(len) = headers.get("Content-Length") {
                let len = len.parse::<usize>()?;
                while buffer.len() < header_end + len {
                    let n = stream.read(&mut tmp).await?;
                    if n == 0 {
                        break;
                    }
                    buffer.extend_from_slice(&tmp[..n]);
                }
            } else if headers
                .get("Transfer-Encoding")
                .map(|v| v.to_ascii_lowercase())
                == Some("chunked".into())
            {
                loop {
                    if buffer[header_end..].windows(5).any(|w| w == b"0\r\n\r\n") {
                        println!("Found chunked terminator!");
                        break;
                    }

                    // Read more data
                    let n = stream.read(&mut tmp).await?;
                    if n == 0 {
                        return Err("connection closed before chunked terminator".into());
                    }
                    buffer.extend_from_slice(&tmp[..n]);
                }

                // Optional: Find exact end position for cleaner termination
                if let Some(terminator_pos) = buffer[header_end..]
                    .windows(5)
                    .position(|w| w == b"0\r\n\r\n")
                {
                    let end_pos = header_end + terminator_pos + 5; // Include the terminator
                    buffer.truncate(end_pos);
                }
            } else {
                println!("connection close");
                // Fallback: read until connection closes
                loop {
                    let n = stream.read(&mut tmp).await?;
                    if n == 0 {
                        break;
                    }
                    buffer.extend_from_slice(&tmp[..n]);
                }
            }

            println!(
                "file size: {} bytes, elapsed: {:?}",
                buffer.len(),
                time.elapsed()
            );
            Ok(buffer)
        }
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", host, e);
            Err(e.into())
        }
    }
}

async fn call_direct() {}
