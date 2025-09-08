use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::Instant;

use reqwest::{Client, Method};

use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
struct ParsedHttpRequest {
    method: Method,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}
#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to server
    let mut stream = TcpStream::connect("127.0.0.1:9090").await?;
    println!("Connected to server!");
    let mut first_message = true;

    // Send message
    let mut buffer = vec![0; 4096];
    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            println!("Server closed connection.");
            break;
        }

        let rec_msg = String::from_utf8_lossy(&buffer[..n]);

        if first_message {
            first_message = !first_message;
            println!("{}", rec_msg);
            continue;
        }

        println!(
            "Received from Server: \n>>> begin\n{}\n>>> end",
            rec_msg.to_string()
        );

        let raw_request = rec_msg.to_string();

        let host = "localhost:3000";
        let response = connect_local_by_reqwest(host, raw_request.as_str())
            .await
            .unwrap();

        let raw_http = response_to_raw(response).await.unwrap();
        stream.write(&raw_http).await?;
    }

    Ok(())
}

async fn response_to_raw(response: reqwest::Response) -> Result<Vec<u8>, Box<dyn Error>> {
    // 1. Status line
    let mut raw = format!(
        "HTTP/1.1 {} {}\r\n",
        response.status().as_u16(),
        response.status().canonical_reason().unwrap_or("")
    );
    // 2. Headers
    for (k, v) in response.headers() {
        raw.push_str(&format!("{}: {}\r\n", k, v.to_str().unwrap_or("")));
    }
    // blank line
    raw.push_str("\r\n");
    // 3. Body (consume it)
    let body = response.bytes().await?;
    let mut raw_bytes = raw.into_bytes();
    raw_bytes.extend_from_slice(&body);

    Ok(raw_bytes)
}

async fn connect_local_by_reqwest(
    host: &str,
    raw_request: &str,
) -> Result<reqwest::Response, Box<dyn Error>> {
    match parse_http_request(raw_request) {
        Ok(parsed) => {
            println!("{:?}", parsed);
            let response = send_with_reqwest(host, parsed).await?;
            Ok(response)
        }
        Err(e) => {
            println!("Parse error: {}", e);
            Err("Invalid Parse Request".into())
        }
    }
}

async fn send_with_reqwest(
    target_host: &str,
    parsed_request: ParsedHttpRequest,
) -> Result<reqwest::Response, Box<dyn Error>> {
    let start = Instant::now();
    // Build URL
    let url = format!("http://{}{}", target_host, parsed_request.path);
    // Create client
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    println!(
        "🔧 Processing {} request to {}",
        parsed_request.method, parsed_request.path
    );

    // Create request builder based on method
    let mut request_builder = match parsed_request.method {
        Method::GET => client.get(&url),
        Method::POST => client.post(&url),
        Method::PUT => client.put(&url),
        Method::DELETE => client.delete(&url),
        Method::PATCH => client.patch(&url),
        Method::HEAD => client.head(&url),
        _ => client.request(parsed_request.method.clone(), &url),
    };

    // Add headers (reqwest handles this much more gracefully)
    for (name, value) in parsed_request.headers {
        let name_lower = name.to_lowercase();

        // Skip headers that reqwest manages automatically
        if !["host", "content-length", "user-agent"].contains(&name_lower.as_str()) {
            request_builder = request_builder.header(&name, &value);
            println!("  ✅ Header: {} = {}", name, value);
        } else if name_lower == "user-agent" {
            // Handle User-Agent specially
            request_builder = request_builder.header("user-agent", &value);
            println!("  ✅ Header: {} = {}", name, value);
        }
    }

    // Add body if exists
    if let Some(body_bytes) = parsed_request.body {
        println!("  📦 Body: {} bytes", body_bytes.len());

        // Auto-detect content type
        if let Ok(body_str) = String::from_utf8(body_bytes.clone()) {
            if body_str.trim_start().starts_with('{') || body_str.trim_start().starts_with('[') {
                println!("  🔍 Detected: JSON body");
                // For JSON, let reqwest handle it
                request_builder = request_builder.body(body_bytes);
            } else if body_str.contains("=") && !body_str.contains("\n") {
                println!("  🔍 Detected: Form data");
                request_builder = request_builder.body(body_bytes);
            } else {
                println!("  🔍 Detected: Text/other body");
                request_builder = request_builder.body(body_bytes);
            }
        } else {
            println!("  🔍 Detected: Binary body");
            request_builder = request_builder.body(body_bytes);
        }
    } else {
        println!("  📦 No body");
    }
    // Send request - much simpler than hyper!
    let response = request_builder.send().await?;
    let elapsed = start.elapsed();
    println!("✅ completed in: {:?}", elapsed);

    Ok(response)
}

// Enhanced parser that handles all body types
fn parse_http_request(raw_request: &str) -> Result<ParsedHttpRequest, Box<dyn Error>> {
    let mut lines = raw_request.lines();

    let request_line = lines.next().ok_or("Empty HTTP request")?;
    let request_parts: Vec<&str> = request_line.split_whitespace().collect();

    if request_parts.len() < 3 {
        return Err("Invalid request line format".into());
    }

    // Convert string to reqwest::Method
    let method = match request_parts[0].to_uppercase().as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "PATCH" => Method::PATCH,
        "HEAD" => Method::HEAD,
        "OPTIONS" => Method::OPTIONS,
        _ => Method::GET, // Default fallback
    };

    let path = request_parts[1].to_string();
    let version = request_parts[2].to_string();

    let mut headers = HashMap::new();

    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }

        if let Some(colon_pos) = line.find(':') {
            let header_name = line[..colon_pos].trim().to_string();
            let header_value = line[colon_pos + 1..].trim().to_string();
            headers.insert(header_name, header_value);
        }
    }

    let body_lines: Vec<&str> = lines.collect();
    let body = if body_lines.is_empty() {
        None
    } else {
        let body_content = body_lines.join("\n");
        if body_content.trim().is_empty() {
            None
        } else {
            Some(body_content.into_bytes())
        }
    };

    Ok(ParsedHttpRequest {
        method,
        path,
        version,
        headers,
        body,
    })
}
