use std::env;
use std::str;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use terminal_size::{Width, terminal_size};
mod monitor;
mod request;
mod scrolling_text;
mod tcp_capture;

use tcp_capture::TcpCapture;

use crate::request::HttpRequest;
use monitor::Monitor;
use scrolling_text::ScrollingText;

const CLIENT_VERSION: &str = "0.1.1";
const HOST_SERVER_TCP: &str = "connl.io:9090";
const HOST_NAME: &str = "connl.io";

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "connl",
    version = CLIENT_VERSION,
    about = "",
    long_about = None,
    disable_version_flag = true,
    override_usage = "
\tconnl [PORT] \t\t\t\texpose localhost with port number"
)]
struct Args {
    #[arg(help = "Port number to expose")]
    port: Option<u16>,

    #[arg(long, help = "Subdomain name for the exposed service")]
    subdomain: Option<String>,

    #[arg(
        long,
        action = clap::ArgAction::SetTrue,
        help = "Show version information"
    )]
    version: bool,
}
#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    // Handle version flag
    if args.version {
        println!("connl v:{}", CLIENT_VERSION);
        return Ok(());
    }

    // Check if port is provided
    let local_port = match args.port {
        Some(port) => port,
        None => {
            Args::parse_from(["connl", "--help"]);
            return Ok(());
        }
    };

    let host_server;
    if env::var("HOST_SERVER_TCP").is_ok() {
        host_server = env::var("HOST_SERVER_TCP").unwrap();
    } else {
        host_server = HOST_SERVER_TCP.to_string();
    }

    // Connect to server
    let mut stream = TcpStream::connect(host_server).await?;
    let mut buffer = [0; 1024];

    // send message first
    let req_connect;
    if let Some(subdomain) = args.subdomain {
        req_connect = format!("connl {} {}", CLIENT_VERSION, subdomain);
    } else {
        req_connect = format!("connl {}", CLIENT_VERSION);
    }

    if let Err(e) = stream.write_all(req_connect.as_bytes()).await {
        println!("Send data to server fails {:?}", e);
        return Ok(());
    }
    if let Err(e) = stream.flush().await {
        eprintln!("Error flushing TCP stream: {}", e);
        return Ok(());
    }

    let n = stream.read(&mut buffer).await?;
    if n == 0 {
        println!("Server Closed Connection.");
    }
    let rec_msg = String::from_utf8_lossy(&buffer[..n]);

    if rec_msg.to_string().to_lowercase().contains("err") {
        let err_code = rec_msg.split(":").nth(0).unwrap_or("Unknown");
        if err_code == "ERR001" {
            println!("please update version : https://connl.io/update_version.html");
        } else {
            println!("Connect Server Error: {}", rec_msg);
        }
        return Ok(());
    }

    Monitor::show_status(rec_msg.to_string(), local_port);

    let screen_w = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    let mut display = ScrollingText::new(4);

    loop {
        let mut total_data = Vec::new();
        let mut status_text: String;
        loop {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                println!("Server Closed Connection.");
                break;
            }
            total_data.extend_from_slice(&buffer[..n]);

            if total_data.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
        }
        let headers_end = total_data
            .windows(4)
            .position(|w| w == b"\r\n\r\n")
            .unwrap()
            + 4;

        let headers_str = str::from_utf8(&total_data[..headers_end - 4]);

        status_text = HttpRequest::parse_content_request_format(
            headers_str.expect("NOT_FOUND_CONTENT_LENGTH"),
        );

        let content_length =
            HttpRequest::parse_content_length(headers_str.expect("NOT_FOUND_CONTENT_LENGTH"));

        if let Some(body_length) = content_length {
            let body_data_received = total_data.len() - headers_end;
            let remaining_body = body_length - body_data_received;
            if remaining_body > 0 {
                let mut body_buf = vec![0u8; remaining_body];
                let mut bytes_read = 0;

                while bytes_read < remaining_body {
                    let n = stream.read(&mut body_buf[bytes_read..]).await?;
                    if n == 0 {
                        println!("Server Closed Connection.");
                    }
                    bytes_read += n;
                }

                total_data.extend_from_slice(&body_buf);
            }
        }

        let host = format!("localhost:{local_port}");

        if let Ok(response) =
            TcpCapture::capture_http_raw(&total_data, host.as_str(), &mut status_text).await
        {
            // before sending data to server
            if let Err(e) = stream.write_all(&response).await {
                println!("Send data to server fails {:?}", e);
                break;
            }
        } else {
            println!("Fail to capture HTTP response");
            let err_connection_refused = "CLIENT_ERROR:ERR_CONNECTION_REFUSED\r\n\r\n";
            status_text = "CLIENT_ERROR:ERR_CONNECTION_REFUSED".to_string();
            if let Err(e) = stream.write_all(&err_connection_refused.as_bytes()).await {
                println!("Send data to server fails {:?}", e);
                break;
            }
        }

        if let Err(e) = stream.flush().await {
            eprintln!("Error flushing TCP stream: {}", e);
            break;
        }

        // print log
        if status_text.len() > screen_w {
            display.append(format!("{}", &status_text[..screen_w]));
        } else {
            display.append(format!("{}", status_text));
        }
    }

    Ok(())
}
