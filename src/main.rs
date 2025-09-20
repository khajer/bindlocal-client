use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;

const PROXY_PORT: u16 = 8080;
const TARGET_PORT: u16 = 3000;
const TARGET_HOST: &str = "127.0.0.1";

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PROXY_PORT))?;
    println!("Proxy server running on port {}", PROXY_PORT);
    println!("Forwarding requests to {}:{}", TARGET_HOST, TARGET_PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(client_stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_client(client_stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(client_stream: TcpStream) -> io::Result<()> {
    // Connect to the target server
    let target_stream = TcpStream::connect(format!("{}:{}", TARGET_HOST, TARGET_PORT))?;

    println!("New connection established");

    // Clone streams for bidirectional forwarding
    let client_to_target = client_stream.try_clone()?;
    let target_to_client = target_stream.try_clone()?;
    let client_stream_clone = client_stream.try_clone()?;
    let target_stream_clone = target_stream.try_clone()?;

    // Forward data from client to target server
    let client_to_target_thread = thread::spawn(move || {
        if let Err(e) = copy_data(client_to_target, target_stream_clone) {
            eprintln!("Error forwarding client to target: {}", e);
        }
    });

    // Forward data from target server to client
    let target_to_client_thread = thread::spawn(move || {
        if let Err(e) = copy_data(target_to_client, client_stream_clone) {
            eprintln!("Error forwarding target to client: {}", e);
        }
    });

    // Wait for both threads to complete
    let _ = client_to_target_thread.join();
    let _ = target_to_client_thread.join();

    println!("Connection closed");
    Ok(())
}

fn copy_data(mut from: TcpStream, mut to: TcpStream) -> io::Result<()> {
    io::copy(&mut from, &mut to)?;
    Ok(())
}
