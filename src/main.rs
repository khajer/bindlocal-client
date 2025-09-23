use std::env;
use std::error::Error;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

mod tcp_capture;
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

        println!("{:?}", buffer);
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
    let request = format!(
        "GET {} HTTP/1.1\r\n\
Host: {}\r\n\
Connection: keep-alive\r\n\
User-Agent: RustTcpClient/1.0\r\n\
Accept: */*\r\n\r\n",
        "/", "localhost:3000"
    );
    let host = "localhost:3000";
    let request_buff = request.as_bytes();
    let response_data = TcpCapture::capture_http_raw(&request_buff, host)
        .await
        .unwrap();

    tokio::fs::write("tmp/response_raw.tcp", &response_data).await?;

    Ok(())
}
