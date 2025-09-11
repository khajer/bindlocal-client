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

        if first_message {
            let rec_msg = String::from_utf8_lossy(&buffer[..n]);
            first_message = !first_message;
            println!("{}", rec_msg);
            continue;
        }
        let rec_msg = String::from_utf8_lossy(&buffer[..n]).to_string();
        println!("request : \n{rec_msg}");

        match TcpStream::connect("localhost:3000").await {
            Ok(mut stream_local) => {
                println!("successfully connected to localhost:3000");

                if let Err(e) = stream_local.write_all(&buffer).await {
                    eprintln!("Error sending direct message to TCP client: {}", e);
                }
                if let Err(e) = stream_local.flush().await {
                    eprintln!("Error flushing TCP stream: {}", e);
                }

                // Buffer to store the response
                let mut response_data: Vec<u8> = Vec::new();
                // Read the response into the vector
                if let Err(e) = stream_local.read_to_end(&mut response_data).await {
                    eprintln!("Error flushing TCP stream: {}", e);
                }
                // stream_local.read_to_end(&mut response_data).await?;
                println!("Response received, length: {} bytes", response_data.len());
                println!("Response received, length: {} bytes", response_data.len());

                println!("{:?}", response_data);
                if let Err(e) = stream.write_all(&response_data).await {
                    println!("Send to server fails {:?}", e);
                }
                if let Err(e) = stream.flush().await {
                    eprintln!("Error flushing TCP stream: {}", e);
                }
                println!(">>> Send Completely");
                println!("Response received, length: {} bytes", response_data.len());
                let rec = String::from_utf8(response_data).unwrap();
                println!("Received data: {}", rec);
            }
            Err(e) => {
                eprintln!("Failed to connect: {}", e);
            }
        };
    }

    Ok(())
}
