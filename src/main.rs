use std::collections::HashMap;

use std::env;
use std::error::Error;
use std::str;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> io::Result<()> {
    let call_local_only = false;
    if call_local_only {
        // call_local().await;
        // // --- Send request ---
        let request = format!(
            "GET {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Connection: keep-alive\r\n\
             User-Agent: RustTcpClient/1.0\r\n\
             Accept: */*\r\n\r\n",
            "/", "localhost:5173"
        );
        // call_2(request.clone()).await.unwrap();
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

        // let host = format!("localhost:{local_port}");

        let request_buff = trim_null_bytes(&buffer);

        let response_data = capture_http_raw1(&request_buff, "localhost:5173")
            .await
            .unwrap();

        if let Err(e) = stream.write_all(&response_data).await {
            println!("Send to server fails {:?}", e);
        }
        println!("send completed.");

        if let Err(e) = stream.flush().await {
            eprintln!("Error flushing TCP stream: {}", e);
        }

        // match TcpStream::connect(host.as_str()).await {
        //     Ok(mut stream_local) => {
        //         println!("connected to {host}");

        //         if let Err(e) = stream_local.write_all(request_buff).await {
        //             eprintln!("Error sending direct message to TCP client: {}", e);
        //         }
        //         if let Err(e) = stream_local.flush().await {
        //             eprintln!("Error flushing TCP stream: {}", e);
        //         }
        //         println!("request completed.");

        //         let mut response_data: Vec<u8> = Vec::new();
        //         if let Err(e) = stream_local.read_to_end(&mut response_data).await {
        //             eprintln!("Error flushing TCP stream: {}", e);
        //         }
        //         println!("response completed.");

        //         if let Err(e) = stream.write_all(&response_data).await {
        //             println!("Send to server fails {:?}", e);
        //         }
        //         println!("send completed.");

        //         if let Err(e) = stream.flush().await {
        //             eprintln!("Error flushing TCP stream: {}", e);
        //         }
        //         println!(">>> OK");
        //         stream_local.shutdown().await?;
        //     }
        //     Err(e) => {
        //         eprintln!("Failed to connect: {}", e);
        //     }
        // };
    }

    Ok(())
}

async fn call_local() {
    let host = "localhost:5173";

    // let buffer_request = [
    //     71, 69, 84, 32, 47, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 72, 111, 115, 116, 58, 32,
    //     48, 48, 48, 49, 46, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 56, 48, 56, 48, 13, 10,
    //     85, 115, 101, 114, 45, 65, 103, 101, 110, 116, 58, 32, 77, 111, 122, 105, 108, 108, 97, 47,
    //     53, 46, 48, 32, 40, 77, 97, 99, 105, 110, 116, 111, 115, 104, 59, 32, 73, 110, 116, 101,
    //     108, 32, 77, 97, 99, 32, 79, 83, 32, 88, 32, 49, 48, 46, 49, 53, 59, 32, 114, 118, 58, 49,
    //     52, 50, 46, 48, 41, 32, 71, 101, 99, 107, 111, 47, 50, 48, 49, 48, 48, 49, 48, 49, 32, 70,
    //     105, 114, 101, 102, 111, 120, 47, 49, 52, 50, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116,
    //     58, 32, 116, 101, 120, 116, 47, 104, 116, 109, 108, 44, 97, 112, 112, 108, 105, 99, 97,
    //     116, 105, 111, 110, 47, 120, 104, 116, 109, 108, 43, 120, 109, 108, 44, 97, 112, 112, 108,
    //     105, 99, 97, 116, 105, 111, 110, 47, 120, 109, 108, 59, 113, 61, 48, 46, 57, 44, 42, 47,
    //     42, 59, 113, 61, 48, 46, 56, 13, 10, 65, 99, 99, 101, 112, 116, 45, 76, 97, 110, 103, 117,
    //     97, 103, 101, 58, 32, 101, 110, 45, 85, 83, 44, 101, 110, 59, 113, 61, 48, 46, 53, 13, 10,
    //     65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105, 110, 103, 58, 32, 103, 122, 105,
    //     112, 44, 32, 100, 101, 102, 108, 97, 116, 101, 44, 32, 98, 114, 44, 32, 122, 115, 116, 100,
    //     13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58, 32, 107, 101, 101, 112, 45, 97,
    //     108, 105, 118, 101, 13, 10, 85, 112, 103, 114, 97, 100, 101, 45, 73, 110, 115, 101, 99,
    //     117, 114, 101, 45, 82, 101, 113, 117, 101, 115, 116, 115, 58, 32, 49, 13, 10, 83, 101, 99,
    //     45, 70, 101, 116, 99, 104, 45, 68, 101, 115, 116, 58, 32, 100, 111, 99, 117, 109, 101, 110,
    //     116, 13, 10, 83, 101, 99, 45, 70, 101, 116, 99, 104, 45, 77, 111, 100, 101, 58, 32, 110,
    //     97, 118, 105, 103, 97, 116, 101, 13, 10, 83, 101, 99, 45, 70, 101, 116, 99, 104, 45, 83,
    //     105, 116, 101, 58, 32, 110, 111, 110, 101, 13, 10, 83, 101, 99, 45, 70, 101, 116, 99, 104,
    //     45, 85, 115, 101, 114, 58, 32, 63, 49, 13, 10, 80, 114, 105, 111, 114, 105, 116, 121, 58,
    //     32, 117, 61, 48, 44, 32, 105, 13, 10, 13, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // ];

    //     let request_browser = b"GET / HTTP/1.1\r\n\
    // Host: localhost:5173\r\n\
    // User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:142.0) Gecko/20100101 Firefox/142.0\r\n\
    // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n\
    // Accept-Language: en-US,en;q=0.5\r\n\
    // Accept-Encoding: gzip, deflate, br, zstd\r\n\
    // Connection: keep-alive\r\n\
    // Upgrade-Insecure-Requests: 1\r\n\
    // Sec-Fetch-Dest: document\r\n\
    // Sec-Fetch-Mode: navigate\r\n\
    // Sec-Fetch-Site: none\r\n\
    // Sec-Fetch-User: ?1\r\n\
    // If-None-Match: \"1ofdq7s\"\r\n\
    // \r\n\
    // \r\n\
    // ";

    //     let buffer_request = b"GET / HTTP/1.1\r\n\
    // Host: 0001.localhost:8080\r\n\
    // User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:142.0) Gecko/20100101 Firefox/142.0\r\n\
    // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n\
    // Accept-Language: en-US,en;q=0.5\r\n\
    // Accept-Encoding: gzip, deflate, br, zstd\r\n\
    // Connection: keep-alive\r\n\
    // Upgrade-Insecure-Requests: 1\r\n\
    // Sec-Fetch-Dest: document\r\n\
    // Sec-Fetch-Mode: navigate\r\n\
    // Sec-Fetch-Site: none\r\n\
    // Sec-Fetch-User: ?1\r\n\
    // Priority: u=0, i\r\n\
    // \r\n\
    // \r\n\
    // ";
    // println!("{:?}", buffer_request1);
    // println!("{:?}", String::from_utf8(buffer_request.to_vec()).unwrap());

    // let buffer_request3 = trim_null_bytes(&buffer_request);

    // let mut buffer = [0; 4096];

    // let buffer_request4 = b"GET / HTTP/1.1\r\n\
    // Host: 0001.localhost:8080\r\n\
    // Connection: Close\r\n\
    // \r\n";

    let buffer_request = b"GET / HTTP/1.1\r\n\
    Host: 0001.localhost:8080\r\n\
    User-Agent: curl/8.7.1\r\n\
    Connection: keep-alive\r\n\
    Accept: */*\r\n\
    \r\n\
    \r\n\
";

    // let buf_request = trim_null_bytes(&buffer_request);
    //
    match TcpStream::connect(host).await {
        Ok(mut stream_local) => {
            // println!("successfully connected to {request}");
            //
            let time = Instant::now();
            if let Err(e) = stream_local.write_all(buffer_request).await {
                eprintln!("Error sending direct message to TCP client: {}", e);
            }
            if let Err(e) = stream_local.flush().await {
                eprintln!("Error flushing TCP stream: {}", e);
            }

            // Read response
            let mut buffer = Vec::new();
            stream_local.read_to_end(&mut buffer).await.unwrap();

            // let mut reader = BufReader::new(&mut stream_local);
            // let mut total_data = Vec::new();
            // let mut buf = vec![0u8; 1024]; //
            // loop {
            //     let n = reader.read(&mut buf).await.unwrap();
            //     if n == 0 {
            //         break; // EOF
            //     }
            //     total_data.extend_from_slice(&buf[..n]);
            //     if total_data.windows(4).any(|w| w == b"\r\n\r\n") {
            //         println!("Complete HTTP headers received");
            //         break;
            //     }
            // }
            // let response_data: Vec<u8> = total_data.to_vec();

            // let str_text = str::from_utf8(&response_data).unwrap();
            // println!("{str_text}");

            // let mut reader = BufReader::new(&mut stream_local);
            // let mut headers = String::new();
            // loop {
            //     let mut line = String::new();
            //     // reader.read_line(&mut line).await;
            //     reader.read_line(&mut line).await.unwrap();
            //     if line == "\r\n" {
            //         break; // end of headers
            //     }
            //     headers.push_str(&line);
            // }
            // println!("Headers:\n{}", headers);

            // let content_length = headers
            //     .lines()
            //     .find(|l| l.to_lowercase().starts_with("content-length"))
            //     .and_then(|l| l.split(':').nth(1))
            //     .map(|s| s.trim().parse::<usize>().ok())
            //     .flatten();

            // let mut body = Vec::new();
            // if let Some(len) = content_length {
            //     // println!("content_length");
            //     let mut buf = vec![0u8; len];
            //     reader.read_exact(&mut buf).await.unwrap();
            //     body.extend_from_slice(&buf);
            // } else if headers.contains("transfer-encoding: chunked") {
            //     // println!("transfer-encoding: chunked");
            // } else {
            //     // println!("read_to_end");
            //     let mut buf = vec![0u8; 1024];
            //     // let mut total_data = Vec::new();
            //     loop {
            //         let n = reader.read(&mut buf).await.unwrap();
            //         if n == 0 {
            //             break; // EOF
            //         }
            //         body.extend_from_slice(&buf[..n]);
            //         if body.windows(4).any(|w| w == b"\r\n\r\n") {
            //             println!("Complete HTTP headers received");
            //             break;
            //         }
            //     }
            //     // reader.read_to_end(&mut body).await.unwrap();
            // }
            // println!("Body length: {}", body.len());
            // let txt = String::from_utf8(body.to_vec()).unwrap();
            // println!("{txt}");

            // println!(
            //     "Response received, length: {} bytes, elapsed time : {:?}",
            //     body.len(),
            //     time.elapsed()
            // );

            // let mut response_data: Vec<u8> = Vec::new();
            // if let Err(e) = stream_local.read_to_end(&mut response_data).await {
            //     eprintln!("Error flushing TCP stream: {}", e);
            // }

            println!(
                "Response received, length: {} bytes, elapsed time : {:?}",
                buffer.len(),
                time.elapsed()
            );

            let rec = String::from_utf8(buffer).unwrap();
            println!("Received data: ===\n{}\n===", rec);
            // println!("Received data: {:?}", response_data);
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    };
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

async fn call_2(request: String) -> Result<(), Box<dyn Error>> {
    // --- Request ---
    let response = send_http_request(request, "localhost:5173").await?;
    println!(
        "Captured {} bytes",
        response.status.len() + response.headers.len() + response.body.len()
    );
    // println!("===== STATUS =====");
    // println!("{}", response.status);

    // println!("\n===== HEADERS =====");
    // for (k, v) in &response.headers {
    //     println!("{}: {}", k, v);
    // }

    // println!("\n===== BODY (as text) =====");
    // println!("{}", String::from_utf8_lossy(&response.body));

    Ok(())
}

#[derive(Debug)]
struct HttpResponse {
    status: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}
async fn send_http_request(request: String, host: &str) -> Result<HttpResponse, Box<dyn Error>> {
    // let port = 80;
    let mut stream = TcpStream::connect(host).await?;

    stream.write_all(request.as_bytes()).await?;
    stream.flush().await?;

    // --- Read until headers end ---
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

    // --- Parse headers ---
    let header_text = String::from_utf8_lossy(&buffer[..header_end]);
    let mut lines = header_text.lines();
    let status_line = lines.next().unwrap_or("").to_string();
    let mut headers = HashMap::new();
    for line in lines {
        if let Some((k, v)) = line.split_once(": ") {
            headers.insert(k.to_string(), v.to_string());
        }
    }

    // --- Read body depending on headers ---
    let mut body = Vec::new();

    if let Some(len) = headers.get("Content-Length") {
        let len = len.parse::<usize>()?;
        body.extend_from_slice(&buffer[header_end..]);
        while body.len() < len {
            let n = stream.read(&mut tmp).await?;
            if n == 0 {
                break;
            }
            body.extend_from_slice(&tmp[..n]);
        }
        body.truncate(len);
    } else if headers
        .get("Transfer-Encoding")
        .map(|v| v.to_ascii_lowercase())
        == Some("chunked".into())
    {
        let mut rest = buffer[header_end..].to_vec();
        loop {
            // Read until we have a full line
            while !rest.windows(2).any(|w| w == b"\r\n") {
                let n = stream.read(&mut tmp).await?;
                if n == 0 {
                    return Err("connection closed during chunk size".into());
                }
                rest.extend_from_slice(&tmp[..n]);
            }

            // Get chunk size
            let pos = rest
                .windows(2)
                .position(|w| w == b"\r\n")
                .expect("chunk line ending");
            let line = String::from_utf8_lossy(&rest[..pos]);
            let size = usize::from_str_radix(line.trim(), 16)?;
            rest.drain(..pos + 2); // remove size line + CRLF

            if size == 0 {
                // Last chunk
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

            // Copy chunk data
            body.extend_from_slice(&rest[..size]);
            rest.drain(..size + 2); // remove chunk + CRLF
        }
    } else {
        // Fallback: read until connection closes
        body.extend_from_slice(&buffer[header_end..]);
        loop {
            let n = stream.read(&mut tmp).await?;
            if n == 0 {
                break;
            }
            body.extend_from_slice(&tmp[..n]);
        }
    }
    println!("call_2 [parse] :{:?}", time.elapsed());
    Ok(HttpResponse {
        status: status_line,
        headers,
        body,
    })
}

async fn call_3(request: String) -> Result<(), Box<dyn Error>> {
    let raw = capture_http_raw(request, "localhost:5173").await?;
    println!("Captured {} bytes", raw.len());
    // println!("{:?}", raw);

    // Save full raw binary
    tokio::fs::write("response_raw.html", &raw).await?;

    Ok(())
}
async fn capture_http_raw(request: String, host: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    // let mut stream = TcpStream::connect((host, 5173)).await?;
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
    // let mut stream = TcpStream::connect((host, 5173)).await?;
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

fn decode_chunked(raw: &[u8]) -> Vec<u8> {
    let mut body = Vec::new();
    let mut rest = raw;

    loop {
        let pos = match rest.windows(2).position(|w| w == b"\r\n") {
            Some(p) => p,
            None => break,
        };

        let size_line = &rest[..pos];
        let size_str = String::from_utf8_lossy(size_line).trim().to_string();

        let size = match usize::from_str_radix(&size_str, 16) {
            Ok(n) => n,
            Err(_) => break,
        };

        rest = &rest[pos + 2..]; // skip size line

        if size == 0 {
            break;
        }

        if rest.len() < size + 2 {
            break; // incomplete
        }

        body.extend_from_slice(&rest[..size]);
        rest = &rest[size + 2..]; // skip data + CRLF
    }

    body
}
// •	We stop reading at headers, instead of waiting for socket close.
// 	•	If Content-Length, we read exactly that many bytes.
// 	•	If chunked, we read until 0\r\n\r\n then decode.
