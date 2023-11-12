use std::fs;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::http::*;

const BUFFER_SIZE: usize = 256;

fn get_file(filename: &str, directory: Arc<String>) -> Option<String> {
    let path = format!("{}/{}", *directory, filename);
    fs::read_to_string(path).ok()
}

fn save_file(filename: &str, directory: Arc<String>, content: String) -> Result<(), String>{
    let path = format!("{}/{}", *directory, filename);

    let mut file = File::create(path).expect("Fail to open file");
    file.write_all(content.as_bytes()).expect("Fail to write to file");

    Ok(())
}


pub fn router(request: HTTPRequest, directory: Arc<String>) -> HTTPResponse {
    let path_parts: Vec<&str> = request.path
        .split("/")
        .filter(|s| !s.is_empty())
        .collect();

    let method = request.method;

    match (method, path_parts.as_slice()) {
        (HTTPMethod::GET, []) => make_http_200_return_text("".to_string()),
        (HTTPMethod::GET, ["echo", parts @ ..]) => {
            let content = parts.join("/");
            make_http_200_return_text(content)
        },
        (HTTPMethod::GET, ["user-agent"]) => {
            let user_agent = request.header.get("user-agent");
            match user_agent {
                Some(agent) => make_http_200_return_text(agent.to_string()),
                None => make_http_404_not_found()
            }
        },
        (HTTPMethod::GET, ["files", parts@ ..]) => {
            let filename = parts.join("/");
            match get_file(&filename, directory) {
                Some(content) => make_http_200_return_file(content),
                None => make_http_404_not_found()
            }
        },
        (HTTPMethod::POST, ["files", parts@ ..]) => {
            let filename = parts.join("/");
            let content = request.body.expect("Missing request body");
            if let Err(_) = save_file(&filename, directory, content) {
                make_http_404_not_found()
            } else {
                make_http_201_created()
            }
        }
        _ => make_http_404_not_found()
    }
}


pub async fn handle_request(mut stream: TcpStream, directory: Arc<String>) -> Result<(), String>{
    let mut buffer = vec![0; BUFFER_SIZE];

    match stream.read(&mut buffer).await {
        Ok(bytes_read) =>{
            let data = String::from_utf8(buffer.into_iter().take(bytes_read).collect())
                .map_err(|e| format!("Error decoding UTF-8: {}", e))?;

            let request = parse_http_request(data.as_str())?;
            let response  = router(request, directory);

            stream.write_all(response.to_string().as_bytes()).await
                .map_err(|e| format!("Failed to write response: {}", e))?;
        }
        Err(e) => {
            return Err(format!("Failed to read data:{}", e));
        }
    }
    Ok(())
}