use std::collections::HashMap;
use itertools::Itertools;

pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}
fn parse_http_method(method: &str) -> Result<HTTPMethod, String> {
    let method = method.to_uppercase();
    match method.as_str() {
        "GET" => Ok(HTTPMethod::GET),
        "POST" => Ok(HTTPMethod::POST),
        "PUT" => Ok(HTTPMethod::PUT),
        "DELETE" => Ok(HTTPMethod::DELETE),
        "HEAD" => Ok(HTTPMethod::HEAD),
        "CONNECT" => Ok(HTTPMethod::CONNECT),
        "OPTIONS" => Ok(HTTPMethod::OPTIONS),
        "TRACE" => Ok(HTTPMethod::TRACE),
        "PATCH" => Ok(HTTPMethod::PATCH),
        _ => Err("Method not supported".to_string())
    }
}
pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub path: String,
    pub version: String,
    pub header: HashMap<String, String>,
    pub body: Option<String>
}
pub fn parse_http_request(request: &str) -> Result<HTTPRequest, String> {
    let mut lines = request.lines();
    let start_line = lines.next().ok_or("Empty request".to_string())?;
    let mut start_line_parts = start_line.split_whitespace();

    // Parse the first line of the request
    let method = parse_http_method(
        start_line_parts.next().ok_or("Invalid request line".to_string())?
    )?;
    let path = start_line_parts.next().ok_or("Invalid request line".to_string())?;
    let version = start_line_parts.next().ok_or("Invalid request line".to_string())?;


    // Parse the request header
    let mut headers = HashMap::new();
    while let Some(line) = lines.next() {
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


    let body = lines.filter(|s| !s.is_empty()).join("\r\n");

    Ok(HTTPRequest{
        method,
        path: path.to_string(),
        version: version.to_string(),
        header: headers,
        body: if !body.is_empty() { Some(body) } else { None }
    })
}
pub enum HTTPResponseStatus {
    Ok,
    NotFound,
}
impl HTTPResponseStatus {
    fn as_str(&self) -> &'static str {
        match self {
            HTTPResponseStatus::Ok => "HTTP/1.1 200 OK\r\n",
            HTTPResponseStatus::NotFound => "HTTP/1.1 404 Not Found\r\n",
        }
    }
}
pub struct HTTPResponse {
    pub status: HTTPResponseStatus,
    pub header: Option<HashMap<String, String>>,
    pub body: Option<String>
}
impl HTTPResponse {
    pub fn to_string(&self) -> String{
        let mut out_string = self.status.as_str().to_string();

        if let Some(header) = &self.header {
            for (k, v) in header.into_iter() {
                out_string.push_str(k);
                out_string.push_str(": ");
                out_string.push_str(v);
                out_string.push_str("\r\n");
            }
        }

        out_string.push_str("\r\n");

        if let Some(body) = &self.body {
            out_string.push_str(body);
            out_string.push_str("\r\n");
        }

        out_string
    }
}

pub fn make_http_404_not_found() -> HTTPResponse {
    return HTTPResponse {
        status: HTTPResponseStatus::NotFound,
        header: None,
        body: None
    }
}

pub fn make_http_200_return_text(content: String) -> HTTPResponse {
    let mut header = HashMap::new();
    header.insert("Content-Type".to_string(), "text/plain".to_string());
    header.insert("Content-Length".to_string(), content.as_bytes().len().to_string());
    return HTTPResponse {
        status: HTTPResponseStatus::Ok,
        header: Some(header),
        body: Some(content)
    }
}

pub fn make_http_200_return_file(content: String) -> HTTPResponse {
    let mut header = HashMap::new();
    header.insert("Content-Type".to_string(), "application/octet-stream".to_string());
    header.insert("Content-Length".to_string(), content.as_bytes().len().to_string());
    return HTTPResponse {
        status: HTTPResponseStatus::Ok,
        header: Some(header),
        body: Some(content)
    }
}