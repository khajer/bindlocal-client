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
        let mut full_buffer = Vec::new();
        stream_client.read_to_end(&mut full_buffer).await?;

        if let Err(e) = stream.write(&full_buffer).await {
            eprintln!("Error sending direct message to server: {}", e);
        }
        if let Err(e) = stream.flush().await {
            eprintln!("Error flushing TCP stream: {}", e);
        }
    }

    Ok(())
}
