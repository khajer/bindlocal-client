use tokio::io;
use monitor::Monitor;
use clap::Parser;

mod connection;
mod monitor;
mod request;
mod request_handler;
mod scrolling_text;
mod tcp_capture;

const CLIENT_VERSION: &str = "0.1.1";
pub const HOST_NAME: &str = "connl.io";

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
        println!("connl v:{CLIENT_VERSION}");
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

    // Connect to server and perform handshake
    let (stream, subdomain) = match connection::Connection::connect_to_server(args.subdomain).await {
        Ok(result) => result,
        Err(e) => {
            println!("{}", e);
            return Ok(());
        }
    };

    Monitor::show_status(subdomain, local_port);

    // Handle request/response loop
    request_handler::RequestHandler::handle_requests(stream, local_port).await
}
