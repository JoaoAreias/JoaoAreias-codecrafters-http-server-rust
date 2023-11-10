use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const BUFFER_SIZE: usize = 255;

enum ResponseStatus {
    Ok,
    NotFound
}

impl ResponseStatus {
    fn as_str(&self) -> &'static str {
        match self {
            ResponseStatus::Ok => "HTTP/1.1 200 OK\r\n\r\n",
            ResponseStatus::NotFound => "HTTP/1.1 404 Not Found\r\n\r\n"
        }
    }
}
struct Response {
    status: ResponseStatus,
    content_type: Option<String>,
    content_length: Option<usize>,
    content: Option<String>
}

impl Response {
    fn as_str(&self) -> String {
        match self.status {
            ResponseStatus::Ok => {
                let default_content_type = "text/plain".to_string();
                let default_content = "".to_string();
                let content_type = self.content_type
                    .as_ref().unwrap_or(&default_content_type);
                let content_length = self.content_length.unwrap_or(0);
                let content = self.content.as_ref()
                    .unwrap_or(&default_content);

                format!(
                    "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                    self.status.as_str(),
                    content_type,
                    content_length,
                    content
                )
            },
            _ => self.status.as_str().to_string()
        }
    }
}

fn resolve_path(path: &str) -> Response {
    let parts: Vec<&str> = path.split("/")
        .filter(|s| !s.is_empty())
        .collect();

    match parts.as_slice() {
        [] => {
            Response {
                status:ResponseStatus::Ok,
                content_type: None,
                content_length: None,
                content: None
            }
        }
        ["echo", text] => {
            Response {
                status: ResponseStatus::Ok,
                content_type: Some("text/plain".to_string()),
                content_length: Some(text.len()),
                content: Some(text.to_string())
            }
        }
        _ => Response {
            status: ResponseStatus::NotFound,
            content_type: None,
            content_length: None,
            content: None
        }
    }
}

fn parse_request(request: &str) -> Result<HashMap<String, String>, String> {
    let valid_methods: HashSet<&str> = [
        "GET",
        // "POST", "PUT", "DELETE",
        // "HEAD", "CONNECT", "OPTIONS",
        // "TRACE", "PATCH"
    ].iter().cloned().collect();


    let mut lines = request.lines();
    let start_line = lines.next().ok_or("Empty request".to_string())?;
    let mut start_line_parts = start_line.split_whitespace();

    let method = start_line_parts.next().ok_or("Invalid request line".to_string())?.to_uppercase();
    let path = start_line_parts.next().ok_or("Invalid request line".to_string())?;
    let version = start_line_parts.next().ok_or("Invalid request line".to_string())?;

    if method.is_empty() || path.is_empty() || version.is_empty() {
        return Err("Invalid request line".to_string());
    }

    if !valid_methods.contains(method.as_str()) {
        return Err(format!("Invalid method {}", method));
    }

    let mut headers = HashMap::new();
    headers.insert("method".to_string(), method);
    headers.insert("path".to_string(), path.to_string());
    headers.insert("version".to_string(), version.to_string());

    // Parsing HTTP headers
    for line in lines.into_iter() {
        if line.is_empty() {
            break; // End of headers
        }

        // Get Header entry
        let (key, value) = line.split_once(":")
            .ok_or_else(|| "Invalid header".to_string())?;

        // Trim whitespace, and handle case insensitivity for key, then insert data into
        // headers dictionary
        headers.insert(
            key.trim().to_lowercase(), // to_lowercase already returns a String
            value.trim().to_string()
        );
    }

    Ok(headers)
}

fn handle_stream(mut stream: TcpStream) -> Result<(), String> {
    let mut buffer = vec![0; BUFFER_SIZE];


    match stream.read(&mut buffer){
        Ok(bytes_read) =>{
            let data = String::from_utf8(buffer.into_iter().take(bytes_read).collect())
                .map_err(|e| format!("Error decoding UTF-8: {}", e))?;

            let headers = parse_request(&data)?;

            let response  = resolve_path(
                headers.get("path").ok_or("Invalid header")?
            );
            stream.write_all(response.as_str().as_bytes())
                .map_err(|e| format!("Failed to write response: {}", e))?;
        }
        Err(e) => {
            return Err(format!("Failed to read data:{}", e));
        }
    }
    Ok(())
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                if let Err(e) = handle_stream(_stream) {
                    println!("Error handling stream: {}", e);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
