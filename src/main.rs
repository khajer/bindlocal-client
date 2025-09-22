use std::collections::HashMap;

use std::env;
use std::error::Error;
use std::str;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> io::Result<()> {
    let call_local_only = true;
    if call_local_only {
        // // --- Send request ---
        let request = format!(
            "GET {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Connection: keep-alive\r\n\
             User-Agent: RustTcpClient/1.0\r\n\
             Accept: */*\r\n\r\n",
            "/", "localhost:3000"
        );
        call_3(request).await.unwrap();
        return Ok(());
    }

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "missing parameter",
        ));
    }
    // let local_port = args[1].as_str();

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

        // let host = format!("localhost:{local_port}");

        let request_buff = trim_null_bytes(&buffer);

        let response_data = capture_http_raw1(&request_buff, "localhost:3000")
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

async fn call_3(request: String) -> Result<(), Box<dyn Error>> {
    let raw = capture_http_raw(request, "localhost:3000").await?;
    println!("Captured {} bytes", raw.len());
    // println!("{:?}", raw);

    // Save full raw binary
    tokio::fs::write("tmp/response_raw.tcp", &raw).await?;

    Ok(())
}
async fn capture_http_raw(request: String, host: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    // let mut stream = TcpStream::connect((host, 3000)).await?;
    println!("Connecting to {}", host);
    match TcpStream::connect(host).await {
        Ok(mut stream) => {
            stream.write_all(request.as_bytes()).await?;
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
            println!(">>>> begin header buffer");
            let txt = String::from_utf8_lossy(&buffer[..header_end]);
            println!("{txt}");
            println!(">>>> end header buffer ");

            // --- Read the body depending on headers ---
            if let Some(len) = headers.get("Content-Length") {
                println!(">> Content-Length");
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
                println!(">> Transfer-Encoding: chunked");
                let mut rest = buffer[header_end..].to_vec();
                loop {
                    // Ensure we have a full line
                    while !rest.windows(2).any(|w| w == b"\r\n") {
                        let n = stream.read(&mut tmp).await?;
                        if n == 0 {
                            return Err("connection closed during chunk size".into());
                        }
                        rest.extend_from_slice(&tmp[..n]);
                    }

                    // Get chunk size
                    let pos = rest.windows(2).position(|w| w == b"\r\n").unwrap();
                    let line = String::from_utf8_lossy(&rest[..pos]);
                    let size = usize::from_str_radix(line.trim(), 16)?;
                    let chunk_header_len = pos + 2;

                    // Copy chunk header into buffer
                    buffer.extend_from_slice(&rest[..chunk_header_len]);
                    rest.drain(..chunk_header_len);

                    if size == 0 {
                        buffer.extend_from_slice(b"\r\n"); // final CRLF
                        break;
                    }

                    // Ensure we have full chunk
                    while rest.len() < size + 2 {
                        let n = stream.read(&mut tmp).await?;
                        if n == 0 {
                            return Err("connection closed during chunk body".into());
                        }
                        rest.extend_from_slice(&tmp[..n]);
                    }

                    // Copy chunk data + CRLF into buffer
                    buffer.extend_from_slice(&rest[..size + 2]);
                    rest.drain(..size + 2);
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
            println!("call_3 [+chunk] {:?}", time.elapsed());
            Ok(buffer)
        }
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", host, e);
            Err(e.into())
        }
    }

    // let mut stream = TcpStream::connect(host).await?;

    // stream.write_all(request.as_bytes()).await?;
    // stream.flush().await?;

    // --- Read until end of headers ---
    // let mut buffer = Vec::new();
    // let mut tmp = [0u8; 1024];
    // let header_end;

    // let time = Instant::now();
    // loop {
    //     let n = stream.read(&mut tmp).await?;
    //     if n == 0 {
    //         return Err("connection closed before headers".into());
    //     }
    //     buffer.extend_from_slice(&tmp[..n]);
    //     if let Some(pos) = buffer.windows(4).position(|w| w == b"\r\n\r\n") {
    //         header_end = pos + 4;
    //         break;
    //     }
    // }

    // --- Parse headers (just enough to know how much to read) ---
    // let header_text = String::from_utf8_lossy(&buffer[..header_end]);
    // let mut headers = HashMap::new();
    // for line in header_text.lines().skip(1) {
    //     if let Some((k, v)) = line.split_once(": ") {
    //         headers.insert(k.to_string(), v.to_string());
    //     }
    // }

    // // --- Read the body depending on headers ---
    // if let Some(len) = headers.get("Content-Length") {
    //     let len = len.parse::<usize>()?;
    //     while buffer.len() < header_end + len {
    //         let n = stream.read(&mut tmp).await?;
    //         if n == 0 {
    //             break;
    //         }
    //         buffer.extend_from_slice(&tmp[..n]);
    //     }
    // } else if headers
    //     .get("Transfer-Encoding")
    //     .map(|v| v.to_ascii_lowercase())
    //     == Some("chunked".into())
    // {
    //     let mut rest = buffer[header_end..].to_vec();
    //     loop {
    //         // Ensure we have a full line
    //         while !rest.windows(2).any(|w| w == b"\r\n") {
    //             let n = stream.read(&mut tmp).await?;
    //             if n == 0 {
    //                 return Err("connection closed during chunk size".into());
    //             }
    //             rest.extend_from_slice(&tmp[..n]);
    //         }

    //         // Get chunk size
    //         let pos = rest.windows(2).position(|w| w == b"\r\n").unwrap();
    //         let line = String::from_utf8_lossy(&rest[..pos]);
    //         let size = usize::from_str_radix(line.trim(), 16)?;
    //         let chunk_header_len = pos + 2;

    //         // Copy chunk header into buffer
    //         buffer.extend_from_slice(&rest[..chunk_header_len]);
    //         rest.drain(..chunk_header_len);

    //         if size == 0 {
    //             buffer.extend_from_slice(b"\r\n"); // final CRLF
    //             break;
    //         }

    //         // Ensure we have full chunk
    //         while rest.len() < size + 2 {
    //             let n = stream.read(&mut tmp).await?;
    //             if n == 0 {
    //                 return Err("connection closed during chunk body".into());
    //             }
    //             rest.extend_from_slice(&tmp[..n]);
    //         }

    //         // Copy chunk data + CRLF into buffer
    //         buffer.extend_from_slice(&rest[..size + 2]);
    //         rest.drain(..size + 2);
    //     }
    // } else {
    //     // Fallback: read until connection closes
    //     loop {
    //         let n = stream.read(&mut tmp).await?;
    //         if n == 0 {
    //             break;
    //         }
    //         buffer.extend_from_slice(&tmp[..n]);
    //     }
    // }
    // println!("call_3 [+chunk] {:?}", time.elapsed());
    // Ok(buffer) // full raw binary response (status + headers + body)
}

async fn capture_http_raw1(request: &[u8], host: &str) -> Result<Vec<u8>, Box<dyn Error>> {
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
                let mut rest = buffer[header_end..].to_vec();
                loop {
                    // Ensure we have a full line
                    while !rest.windows(2).any(|w| w == b"\r\n") {
                        let n = stream.read(&mut tmp).await?;
                        if n == 0 {
                            return Err("connection closed during chunk size".into());
                        }
                        rest.extend_from_slice(&tmp[..n]);
                    }

                    // Get chunk size
                    let pos = rest.windows(2).position(|w| w == b"\r\n").unwrap();
                    let line = String::from_utf8_lossy(&rest[..pos]);
                    let size = usize::from_str_radix(line.trim(), 16)?;
                    let chunk_header_len = pos + 2;

                    // Copy chunk header into buffer
                    buffer.extend_from_slice(&rest[..chunk_header_len]);
                    rest.drain(..chunk_header_len);

                    if size == 0 {
                        buffer.extend_from_slice(b"\r\n"); // final CRLF
                        break;
                    }

                    // Ensure we have full chunk
                    while rest.len() < size + 2 {
                        let n = stream.read(&mut tmp).await?;
                        if n == 0 {
                            return Err("connection closed during chunk body".into());
                        }
                        rest.extend_from_slice(&tmp[..n]);
                    }

                    // Copy chunk data + CRLF into buffer
                    buffer.extend_from_slice(&rest[..size + 2]);
                    rest.drain(..size + 2);
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
                "[+chunk] size: {} bytes, elapsed: {:?}",
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

// •	We stop reading at headers, instead of waiting for socket close.
// 	•	If Content-Length, we read exactly that many bytes.
// 	•	If chunked, we read until 0\r\n\r\n then decode.
