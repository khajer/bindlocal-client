use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::Instant;

use std::env;

#[tokio::main]
async fn main() -> io::Result<()> {
    call_local().await;

    // let args: Vec<String> = env::args().collect();

    // if args.len() < 1 {
    //     return Err(io::Error::new(
    //         io::ErrorKind::InvalidInput,
    //         "missing parameter",
    //     ));
    // }
    // let local_port = args[1].as_str();

    // // Connect to server
    // let mut stream = TcpStream::connect("127.0.0.1:9090").await?;
    // let mut first_message = true;
    // let mut buffer = [0; 1024];
    // loop {
    //     let n = stream.read(&mut buffer).await?;
    //     if n == 0 {
    //         println!("Server Closed Connection.");
    //         break;
    //     }

    //     if first_message {
    //         let rec_msg = String::from_utf8_lossy(&buffer[..n]);
    //         first_message = !first_message;
    //         println!("{}", rec_msg);
    //         continue;
    //     }

    //     println!("{:?}", buffer);
    //     let rec_msg = String::from_utf8_lossy(&buffer[..n]).to_string();
    //     println!("request : \n{rec_msg}");

    //     let host = format!("localhost:{local_port}");

    //     match TcpStream::connect(host.as_str()).await {
    //         Ok(mut stream_local) => {
    //             println!("successfully connected to {host}");

    //             if let Err(e) = stream_local.write_all(&buffer).await {
    //                 eprintln!("Error sending direct message to TCP client: {}", e);
    //             }
    //             if let Err(e) = stream_local.flush().await {
    //                 eprintln!("Error flushing TCP stream: {}", e);
    //             }

    //             let mut response_data: Vec<u8> = Vec::new();
    //             if let Err(e) = stream_local.read_to_end(&mut response_data).await {
    //                 eprintln!("Error flushing TCP stream: {}", e);
    //             }

    //             println!("Response received, length: {} bytes", response_data.len());
    //             let rec = String::from_utf8(response_data.clone()).unwrap();
    //             println!("Received data: {}", rec);

    //             if let Err(e) = stream.write_all(&response_data).await {
    //                 println!("Send to server fails {:?}", e);
    //             }
    //             if let Err(e) = stream.flush().await {
    //                 eprintln!("Error flushing TCP stream: {}", e);
    //             }
    //             println!(">>> Send Completely");
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to connect: {}", e);
    //         }
    //     };
    // }

    Ok(())
}

async fn call_local() {
    let host = "localhost:5173";

    let buffer_request = [
        71, 69, 84, 32, 47, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 72, 111, 115, 116, 58, 32,
        48, 48, 48, 49, 46, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 56, 48, 56, 48, 13, 10,
        85, 115, 101, 114, 45, 65, 103, 101, 110, 116, 58, 32, 77, 111, 122, 105, 108, 108, 97, 47,
        53, 46, 48, 32, 40, 77, 97, 99, 105, 110, 116, 111, 115, 104, 59, 32, 73, 110, 116, 101,
        108, 32, 77, 97, 99, 32, 79, 83, 32, 88, 32, 49, 48, 46, 49, 53, 59, 32, 114, 118, 58, 49,
        52, 50, 46, 48, 41, 32, 71, 101, 99, 107, 111, 47, 50, 48, 49, 48, 48, 49, 48, 49, 32, 70,
        105, 114, 101, 102, 111, 120, 47, 49, 52, 50, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116,
        58, 32, 116, 101, 120, 116, 47, 104, 116, 109, 108, 44, 97, 112, 112, 108, 105, 99, 97,
        116, 105, 111, 110, 47, 120, 104, 116, 109, 108, 43, 120, 109, 108, 44, 97, 112, 112, 108,
        105, 99, 97, 116, 105, 111, 110, 47, 120, 109, 108, 59, 113, 61, 48, 46, 57, 44, 42, 47,
        42, 59, 113, 61, 48, 46, 56, 13, 10, 65, 99, 99, 101, 112, 116, 45, 76, 97, 110, 103, 117,
        97, 103, 101, 58, 32, 101, 110, 45, 85, 83, 44, 101, 110, 59, 113, 61, 48, 46, 53, 13, 10,
        65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105, 110, 103, 58, 32, 103, 122, 105,
        112, 44, 32, 100, 101, 102, 108, 97, 116, 101, 44, 32, 98, 114, 44, 32, 122, 115, 116, 100,
        13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58, 32, 107, 101, 101, 112, 45, 97,
        108, 105, 118, 101, 13, 10, 85, 112, 103, 114, 97, 100, 101, 45, 73, 110, 115, 101, 99,
        117, 114, 101, 45, 82, 101, 113, 117, 101, 115, 116, 115, 58, 32, 49, 13, 10, 83, 101, 99,
        45, 70, 101, 116, 99, 104, 45, 68, 101, 115, 116, 58, 32, 100, 111, 99, 117, 109, 101, 110,
        116, 13, 10, 83, 101, 99, 45, 70, 101, 116, 99, 104, 45, 77, 111, 100, 101, 58, 32, 110,
        97, 118, 105, 103, 97, 116, 101, 13, 10, 83, 101, 99, 45, 70, 101, 116, 99, 104, 45, 83,
        105, 116, 101, 58, 32, 110, 111, 110, 101, 13, 10, 83, 101, 99, 45, 70, 101, 116, 99, 104,
        45, 85, 115, 101, 114, 58, 32, 63, 49, 13, 10, 80, 114, 105, 111, 114, 105, 116, 121, 58,
        32, 117, 61, 48, 44, 32, 105, 13, 10, 13, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let buffer_request1 = b"GET / HTTP/1.1\r\n\
Host: 0001.localhost:8080\r\n\
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:142.0) Gecko/20100101 Firefox/142.0\r\n\
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n\
Accept-Language: en-US,en;q=0.5\r\n\
Accept-Encoding: gzip, deflate, br, zstd\r\n\
Connection: keep-alive\r\n\
Upgrade-Insecure-Requests: 1\r\n\
Sec-Fetch-Dest: document\r\n\
Sec-Fetch-Mode: navigate\r\n\
Sec-Fetch-Site: none\r\n\
Sec-Fetch-User: ?1\r\n\
Priority: u=0, i\r\n\
\r\n\
\r\n\
";
    // println!("{:?}", buffer_request1);
    // println!("{:?}", String::from_utf8(buffer_request.to_vec()).unwrap());

    // let buffer_request3 = trim_null_bytes(&buffer_request);

    // let mut buffer = [0; 4096];

    let buffer_request4 = b"GET / HTTP/1.1\r\n\
Host: 0001.localhost:8080\r\n\
Connection: keep-alive\r\n\
\r\n";

    //     let buffer_request4 = b"GET / HTTP/1.1\r\n\
    // Host: 0001.localhost:8080\r\n\
    // User-Agent: curl/8.7.1\r\n\
    // Connection: keep-alive\r\n\
    // Accept: */*\r\n\
    // \r\n\
    // \r\n\
    //     ";

    match TcpStream::connect(host).await {
        Ok(mut stream_local) => {
            // println!("successfully connected to {request}");
            //
            let time = Instant::now();
            if let Err(e) = stream_local.write_all(buffer_request4).await {
                eprintln!("Error sending direct message to TCP client: {}", e);
            }
            if let Err(e) = stream_local.flush().await {
                eprintln!("Error flushing TCP stream: {}", e);
            }

            let mut response_data: Vec<u8> = Vec::new();
            if let Err(e) = stream_local.read_to_end(&mut response_data).await {
                eprintln!("Error flushing TCP stream: {}", e);
            }

            println!(
                "Response received, length: {} bytes, elapsed time : {:?}",
                response_data.len(),
                time.elapsed()
            );

            // let rec = String::from_utf8(response_data.clone()).unwrap();
            // println!("Received data: {}", rec);
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    };
}

fn trim_null_bytes(data: &[u8]) -> &[u8] {
    let start = data.iter().position(|&b| b != 0).unwrap_or(data.len());
    let end = data.iter().rposition(|&b| b != 0).unwrap_or(data.len());
    if start >= data.len() {
        &data[0..0]
    } else {
        // Slice the original data. Add +1 to the end index because it's non-inclusive.
        &data[start..=end]
    }
}
