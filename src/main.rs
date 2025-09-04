use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to server
    let mut stream = TcpStream::connect("127.0.0.1:9090").await?;
    println!("Connected to server!");
    let mut first_message = true;

    // Send message
    let mut buffer = [0; 1024];
    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            println!("Server closed connection.");
            break;
        }

        let rec_msg = String::from_utf8_lossy(&buffer[..n]);

        if first_message {
            first_message = !first_message;
            println!("msg: \n{}", rec_msg);
            continue;
        }

        println!("Received: {}", rec_msg.to_string());

        let raw_request = rec_msg.to_string();
        let mut stream_client = TcpStream::connect("localhost:3000").await?;

        // Send request
        stream_client.write_all(raw_request.as_bytes()).await?;
        stream_client.flush().await?;

        // Read response
        let mut buffer = vec![0; 4096];
        let nn = stream_client.read(&mut buffer).await?;
        let rec_client_msg = String::from_utf8_lossy(&buffer[..nn]);
        println!("Received message from localhost: {}", rec_client_msg);
        stream.write_all(rec_client_msg.as_bytes()).await?;
    }

    Ok(())
}
