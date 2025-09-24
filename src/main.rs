use std::env;
use std::error::Error;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

mod tcp_capture;
use chrono::{Datelike, Local, Timelike};
use tcp_capture::TcpCapture;

#[tokio::main]
async fn main() -> io::Result<()> {
    if env::var("LOCAL_DEV").is_ok() {
        let local_dev = env::var("LOCAL_DEV").unwrap();
        if local_dev.to_lowercase() == "true".to_string() {
            call_direct().await.unwrap();
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

        let rec_msg = String::from_utf8_lossy(&buffer[..n]).to_string();
        println!("request : \n{rec_msg}");

        let host = format!("localhost:{local_port}");
        let request_buff = trim_null_bytes(&buffer);
        let response_data = TcpCapture::capture_http_raw(&request_buff, host.as_str())
            .await
            .unwrap();

        if let Err(e) = stream.write_all(&response_data).await {
            println!("Send to server fails {:?}", e);
        }

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

async fn call_direct() -> Result<(), Box<dyn Error>> {
    let request = b"GET / HTTP/1.1\r\n
Host: 0001.localhost:8080\r\n
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:143.0) Gecko/20100101 Firefox/143.0\r\n
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n
Accept-Language: en-US,en;q=0.5\r\n
Accept-Encoding: gzip, deflate, br, zstd\r\n
Connection: keep-alive\r\n
Upgrade-Insecure-Requests: 1\r\n
Sec-Fetch-Dest: document\r\n
Sec-Fetch-Mode: navigate\r\n
Sec-Fetch-Site: none\r\n
Sec-Fetch-User: ?1\r\n
If-None-Match: W/\"6af-+M4OSPFNZpwKBdFEydrj+1+V5xo\"\r\n
Priority: u=0, i\r\n\r\n";

    let host = "localhost:3000";
    let request_buff = request.to_vec();
    let response_data = TcpCapture::capture_http_raw(&request_buff, host)
        .await
        .unwrap();

    let now = Local::now();
    let filename = format!(
        "tmp/response_{}-{}-{} {}:{}:{}.tcp",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
    tokio::fs::write(filename, &response_data).await?;

    Ok(())
}
