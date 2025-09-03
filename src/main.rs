use rand::Rng;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to server
    let mut stream = TcpStream::connect("127.0.0.1:9090").await?;
    println!("Connected to server!");

    // Send message
    // stream.write_all(b"Hello from client!\n").await?;
    //
    // stream.write_all(b"echo Test!\n").await?;

    let mut buffer = [0; 1024];
    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            println!("Server closed connection.");
            break;
        }

        let rec_msg = String::from_utf8_lossy(&buffer[..n]);
        println!("Received: {}", rec_msg);

        if rec_msg == "PING" {
            let mut rng = rand::thread_rng();
            let random_int: u32 = rng.gen_range(0..100);

            let msg = format!("value = {}", random_int);
            println!("Sent: {}", msg);
            stream.write_all(msg.as_bytes()).await?;
            // stream.write_all(b"PONG\n").await?;
        }
    }

    Ok(())
}
