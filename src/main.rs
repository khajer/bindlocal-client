use futures::{SinkExt, StreamExt};
use tokio::io::{self, AsyncBufReadExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://127.0.0.1:8080";
    println!("Connecting to: {}", url);
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected to WebSocket server");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let read_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(message) => match message {
                    Message::Text(text) => {
                        println!("Received: {}", text);
                    }
                    Message::Binary(data) => {
                        println!("Received {} bytes of binary data", data.len());
                    }
                    Message::Close(_) => {
                        println!("Server closed connection");
                        break;
                    }
                    _ => {}
                },
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    });

    // Handle user input
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    println!("Type messages to send (or 'quit' to exit):");
    while let Some(line) = stdin.next_line().await? {
        let line = line.trim();

        if line == "quit" {
            break;
        }

        if !line.is_empty() {
            if let Err(e) = ws_sender.send(Message::Text(line.to_string())).await {
                eprintln!("Error sending message: {}", e);
                break;
            }
        }
    }

    // Close the connection
    let _ = ws_sender.send(Message::Close(None)).await;

    // Wait for the read task to finish
    let _ = read_task.await;

    println!("Connection closed");
    Ok(())
}
