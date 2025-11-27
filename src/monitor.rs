use crate::CLIENT_VERSION;
use crate::HOST_NAME;
use colored::Colorize;
use std::env;
pub struct Monitor {}

impl Monitor {
    pub fn show_status(url: String, local_port: u16) {
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
}
