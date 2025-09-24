use std::collections::HashMap;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::Instant;

pub struct TcpCapture {}
impl TcpCapture {
    pub async fn capture_http_raw(request: &[u8], host: &str) -> Result<Vec<u8>, Box<dyn Error>> {
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
                    loop {
                        if buffer[header_end..].windows(4).any(|w| w == b"\r\n\r\n") {
                            break;
                        }

                        // Read more data
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
}
