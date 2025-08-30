use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to server
    let mut stream = TcpStream::connect("127.0.0.1:9090").await?;
    println!("Connected to server!");

    // Send message
    stream.write_all(b"Hello from client!\n").await?;

    let mut buffer = [0; 1024];
    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            println!("Server closed connection.");
            break;
        }
        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    }

    Ok(())
}
