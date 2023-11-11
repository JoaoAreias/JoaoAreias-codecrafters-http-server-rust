mod http;
mod handler;

use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Parse command line arguments, codecrafters do not use Clap
    let mut args = env::args();
    let mut directory = ".".to_string();

    while let Some(arg) = args.next() {
        if arg == "--directory" {
            directory = args.next().unwrap_or(".".to_string());
        }
    }

    let directory = Arc::new(directory);

    // HTTP Server
    println!("Server is starting...");
    let listener = match TcpListener::bind("127.0.0.1:4221").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to the port: {}", e);
            return;
        }
    };

    loop {
        let (stream, addr) = match listener.accept().await {
            Ok((stream, addr)) => (stream, addr),
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                continue;
            }
        };

        println!("Accepting connection from {}", addr);
        let directory_clone = directory.clone();
        tokio::spawn(async move {
            if let Err(e) = handler::handle_request(stream, directory_clone).await {
                println!("Connection with {} failed: {}", addr, e);
            }
        });
    }
}
