use std::env;
use std::str;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use terminal_size::{Width, terminal_size};
mod request;
mod scrolling_text;
mod tcp_capture;

use tcp_capture::TcpCapture;

use crate::request::HttpRequest;
use colored::Colorize;
use scrolling_text::ScrollingText;

const CLIENT_VERSION: &str = "0.1.0";
const HOST_SERVER_TCP: &str = "connl.io:9090";
const HOST_NAME: &str = "connl.io";

#[tokio::main]
async fn main() -> io::Result<()> {
    let host_server;
    if env::var("HOST_SERVER_TCP").is_ok() {
        host_server = env::var("HOST_SERVER_TCP").unwrap();
    } else {
        host_server = HOST_SERVER_TCP.to_string();
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        show_help();
        return Ok(());
    }
    let local_port = args[1].as_str();
    if local_port == "version" {
        println!("connl version {}", CLIENT_VERSION);
        return Ok(());
    }

    // Connect to server
    let mut stream = TcpStream::connect(host_server).await?;
    let mut buffer = [0; 1024];

    // send message first

    // specific name sub-domain
    let req_connect;
    if args.len() >= 4 && args[2].as_str() == "--subdomain" {
        req_connect = format!("connl {} {}", CLIENT_VERSION, args[3].as_str());
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

    show_monitor(
        rec_msg.to_string(),
        local_port.parse::<u16>().expect("Failed to parse to u16"),
    );

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

fn show_help() {
    println!(
        "connl v:{CLIENT_VERSION}
usage:\tconnl <port>]\t\t\t\texpose localhost with port number
\tconnl <port> --subdomain <myapp>\texpose localhost with port number and <myapp> subdomain
\tconnl version\t\t\t\tshow version"
    );
}

fn show_monitor(url: String, local_port: u16) {
    let host;
    if env::var("HOST_SERVER_HTTP").is_ok() {
        host = env::var("HOST_SERVER_HTTP").unwrap();
    } else {
        host = HOST_NAME.to_string();
    }
    let txt = format!(
        "
connl.io v:{CLIENT_VERSION}
\t{}\t\t\t\t\t{}
{}\thttp://{url}.{host}\t-> \thttp://localhost:{local_port}
{}\thttps://{url}.{host}\t-> \thttp://localhost:{local_port}
--",
        "online".green(),
        "local".green(),
        "http:".green(),
        "https:".green()
    );
    println!("{txt}");
}
