mod http;
mod handler;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
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

        tokio::spawn(async move {
            if let Err(e) = handler::handle_request(stream).await {
                println!("Connection with {} failed: {}", addr, e);
            }
        });
    }
}
