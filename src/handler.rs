use std::fs;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::http::*;

const BUFFER_SIZE: usize = 256;

fn get_file(filename: &str, directory: Arc<String>) -> Option<String> {
    let path = format!("{}/{}", *directory, filename);
    fs::read_to_string(path).ok()
}


pub fn router(request: HTTPRequest, directory: Arc<String>) -> HTTPResponse {
    let path_parts: Vec<&str> = request.path
        .split("/")
        .filter(|s| !s.is_empty())
        .collect();

    match path_parts.as_slice() {
        [] => make_http_response(HTTPResponseStatus::Ok, None),
        ["echo", parts @ ..] => {
            let content = parts.join("/");
            make_http_response(HTTPResponseStatus::Ok, Some(content))
        },
        ["user-agent"] => {
            let user_agent = request.header.get("user-agent");
            match user_agent {
                Some(agent) => make_http_response(
                    HTTPResponseStatus::Ok,
                    Some(agent.to_string())
                ),
                None => make_http_response(
                    HTTPResponseStatus::NotFound,
                    None
                )
            }
        },
        ["files", parts@ ..] => {
            let filename = parts.join("/");
            match get_file(&filename, directory) {
                Some(content) => make_http_response(
                    HTTPResponseStatus::Ok,
                    Some(content)
                ),
                None => make_http_response(
                    HTTPResponseStatus::NotFound,
                    None
                )
            }

        }
        _ => make_http_response(HTTPResponseStatus::NotFound, None)
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