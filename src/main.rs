use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to server
    let mut stream = TcpStream::connect("127.0.0.1:9090").await?;

    let mut first_message = true;
    // Send message
    let mut buffer = [0; 4096];
    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            println!("Server Closed Connection.");
            break;
        }

        let rec_msg = String::from_utf8_lossy(&buffer[..n]);

        if first_message {
            first_message = !first_message;
            println!("{}", rec_msg);
            continue;
        }

        let raw_request = rec_msg.to_string();

        match TcpStream::connect("localhost:5173").await {
            Ok(stream_local) => {
                let mut stream_client = stream_local;
                println!("successfully connected to localhost:5173");

                stream_client.write(raw_request.as_bytes()).await?;

                let mut full_buffer = vec![0u8; 4096]; // Initial capacity
                let mut total_data = Vec::new();
                loop {
                    let n = stream_client.read(&mut full_buffer).await?;
                    if n == 0 {
                        break; // EOF
                    }
                    total_data.extend_from_slice(&full_buffer[..n]);

                    if total_data.windows(4).any(|w| w == b"\r\n\r\n") {
                        // println!("Complete HTTP headers received");
                        break;
                    }
                }
                if let Err(e) = stream.write(&full_buffer).await {
                    eprintln!("Error sending direct message to server: {}", e);
                }
                if let Err(e) = stream.flush().await {
                    eprintln!("Error flushing TCP stream: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to connect: {}", e);
            }
        };
    }

    Ok(())
}
