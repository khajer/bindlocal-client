use std::str;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use terminal_size::{Width, terminal_size};
use crate::tcp_capture::TcpCapture;
use crate::request::HttpRequest;
use crate::scrolling_text::ScrollingText;
use crate::logger;

const CLIENT_ERROR: &str = "CLIENT_ERROR:ERR_CONNECTION_REFUSED";
const NOT_FOUND_CONTENT_LENGTH: &str = "CLIENT_ERROR:NOT_FOUND_CONTENT_LENGTH";
const TWO_DELIMETER: &str = "\r\n\r\n";
const TWO_DELIMETER_BYTES: &[u8] = b"\r\n\r\n";

pub struct RequestHandler;

impl RequestHandler {
    /// Handles the request/response loop between the tunnel server and local application
    pub async fn handle_requests(
        mut stream: TcpStream,
        local_port: u16,
    ) -> io::Result<()> {
        let screen_w = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(80);

        let mut display = ScrollingText::new(4);
        let mut buffer = [0; 4096];

        loop {
            let mut total_data = Vec::new();
            let mut status_text: String;

            // Read HTTP headers
            loop {
                let n = stream.read(&mut buffer).await?;
                if n == 0 {
                    println!("Server Closed Connection.");
                    logger::log("Server Closed Connection.");
                    return Ok(());
                }
                total_data.extend_from_slice(&buffer[..n]);

                if total_data.windows(4).any(|w| w == TWO_DELIMETER_BYTES) {
                    break;
                }
            }

            let headers_end = total_data
                .windows(4)
                .position(|w| w == TWO_DELIMETER_BYTES)
                .unwrap()
                + 4;

            let headers_str = str::from_utf8(&total_data[..headers_end - 4]);

            status_text = HttpRequest::parse_content_request_format(
                headers_str.expect(NOT_FOUND_CONTENT_LENGTH)
            );

            let content_length = HttpRequest::parse_content_length(
                headers_str.expect(NOT_FOUND_CONTENT_LENGTH)
            );

            // Read request body if Content-Length is present
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
                    logger::log("Server Closed Connection.");
                            return Ok(());
                        }
                        bytes_read += n;
                    }

                    total_data.extend_from_slice(&body_buf);
                }
            }

            let host = format!("localhost:{local_port}");

            // Forward request to local application and capture response
            if let Ok(response) =
                TcpCapture::capture_http_raw(&total_data, host.as_str(), &mut status_text).await
            {
                stream.write_all(&response).await.map_err(|e| {
                    io::Error::new(io::ErrorKind::Other, format!("Send data to server fails: {:?}", e))
                })?;
            } else {
                println!("Fail to capture HTTP response");
                logger::log("Fail to capture HTTP response");
                let err_connection_refused = format!("{CLIENT_ERROR}{TWO_DELIMETER}");
                status_text = CLIENT_ERROR.to_string();

                stream.write_all(err_connection_refused.as_bytes()).await.map_err(|e| {
                    io::Error::new(io::ErrorKind::Other, format!("Send data to server fails: {:?}", e))
                })?;
            }

            stream.flush().await.map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Error flushing TCP stream: {}", e))
            })?;

            logger::log(&status_text);

            // Display status in terminal
            if status_text.len() > screen_w {
                display.append(format!("{}", &status_text[..screen_w]));
            } else {
                display.append(format!("{status_text}"));
            }
        }
    }
}
