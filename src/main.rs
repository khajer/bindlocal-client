use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to server
    let mut stream = TcpStream::connect("127.0.0.1:9090").await?;
    println!("Connected to server!");
    let mut first_message = true;

    // Send message
    let mut buffer = [0; 4096];
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

        let raw_request = rec_msg.to_string();
        let mut stream_client = TcpStream::connect("localhost:3000").await?;

        // send request client
        stream_client.write_all(raw_request.as_bytes()).await?;
        stream_client.flush().await?;

        // Read response client
        // let mut full_buffer = Vec::new();
        // stream_client.read_to_end(&mut full_buffer).await?;

        let mut full_buffer = vec![0u8; 4096]; // Initial capacity
        let mut total_data = Vec::new();
        loop {
            println!("start");
            let n = stream_client.read(&mut full_buffer).await?;
            println!("read");
            if n == 0 {
                break; // EOF
            }
            println!("loop");
            total_data.extend_from_slice(&full_buffer[..n]);
            println!("extend");

            if total_data.windows(4).any(|w| w == b"\r\n\r\n") {
                println!("Complete HTTP headers received");
                break;
            }
        }
        let s = match String::from_utf8(total_data) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{s}");

        if let Err(e) = stream.write(&full_buffer).await {
            eprintln!("Error sending direct message to server: {}", e);
        }
        if let Err(e) = stream.flush().await {
            eprintln!("Error flushing TCP stream: {}", e);
        }
    }

    Ok(())
}
