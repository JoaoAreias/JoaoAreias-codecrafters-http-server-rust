use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


fn handle_stream(mut stream: TcpStream) -> std::io::Result<()>{
    let mut buffer = vec![0; 255];
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let bytes_read= stream.read(buffer.as_mut_slice());


    match bytes_read {
        Ok(_) =>{
            println!("{}", String::from_utf8(buffer).unwrap());
            let _ = stream.write(response.as_bytes());
        }
        Err(e) => {
            return Err(e);
        }
    }
    return Ok(())
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                let _ = handle_stream(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
