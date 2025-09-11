use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    // let raw_request = b"GET / HTTP/1.1\r\n
    // Host: 0001.localhost:8080\r\n
    // User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:142.0) Gecko/20100101 Firefox/142.0\r\n
    // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n
    // Accept-Language: en-US,en;q=0.5\r\n
    // Accept-Encoding: gzip, deflate, br, zstd\r\n
    // Connection: keep-alive\r\n
    // Upgrade-Insecure-Requests: 1\r\n
    // Sec-Fetch-Dest: document\r\n
    // Sec-Fetch-Mode: navigate\r\n
    // Sec-Fetch-Site: none\r\n
    // Sec-Fetch-User: ?1\r\n
    // Priority: u=0, i\r\n";
    // call_local(raw_request).await;

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

        // let raw_request = rec_msg.to_string();
        // call_local(&buffer).await;

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
                println!("Completed");
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

async fn call_local(raw_request: &[u8]) {
    //     let raw_request = b"GET / HTTP/1.1\r\n
    //     Host: 0001.localhost:8080\r\n
    //     User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:142.0) Gecko/20100101 Firefox/142.0\r\n
    //     Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n
    //     Accept-Language: en-US,en;q=0.5\r\n
    //     Accept-Encoding: gzip, deflate, br, zstd\r\n
    //     Connection: keep-alive\r\n
    //     Upgrade-Insecure-Requests: 1\r\n
    //     Sec-Fetch-Dest: document\r\n
    //     Sec-Fetch-Mode: navigate\r\n
    //     Sec-Fetch-Site: none\r\n
    //     Sec-Fetch-User: ?1\r\n
    //     Priority: u=0, i\r\n
    // \r\n";

    match TcpStream::connect("localhost:3000").await {
        Ok(mut stream_local) => {
            // let mut stream_client = stream_local;
            println!("successfully connected to localhost:3000");

            if let Err(e) = stream_local.write_all(raw_request).await {
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
            let rec = String::from_utf8(response_data).unwrap();
            println!("Received data: {}", rec);
        }
        _ => {}
    }
}
