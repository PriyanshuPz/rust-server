use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;
#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub content: String,
}
const MESSAGE_SIZE: usize = 1024;

impl Request {
    pub fn new(mut stream: &TcpStream) -> Result<Self, String> {
        // full request recieved
        let mut received: Vec<u8> = vec![];

        // request pieces in bytes vec
        let mut rx_bytes = [0u8; MESSAGE_SIZE];

        loop {
            // Read from the current data in the TcpStream
            let bytes_read = stream.read(&mut rx_bytes);
            match bytes_read {
                Ok(bytes) => {
                    // However many bytes we read, extend the `received` string bytes
                    received.extend_from_slice(&rx_bytes[..bytes]);
                    // If we didn't fill the array
                    // stop reading because there's no more data (we hope!)
                    if bytes < MESSAGE_SIZE {
                        break;
                    }
                }
                Err(err) => {
                    println!("error: {:#?}", err);
                }
            }
        }
        let request_text = String::from_utf8(received).unwrap();

        // Split request lines (keeping newline characters)
        let mut request_lines: Vec<&str> = request_text.split_inclusive('\n').collect();

        // Store headers and query parameters
        let mut header_map: HashMap<String, String> = HashMap::new();
        let mut query_params: HashMap<String, String> = HashMap::new();

        // First line of HTTP request, e.g., "GET /path?x=1 HTTP/1.1"
        let request_line = request_lines[0];
        let mut parts = request_line.split_ascii_whitespace();
        // Extract HTTP method and full path
        let http_method = parts.next().unwrap();
        let full_path = parts.next().unwrap();

        // Separate path and query string (e.g., "/search?q=rust")
        let path_and_query: Vec<&str> = full_path.split('?').collect();
        let path = path_and_query[0];

        if path_and_query.len() > 1 {
            let query_string = path_and_query[1..].join("");

            let query_pairs: Vec<&str> = query_string.split("&").collect();

            for pair in query_pairs {
                if let Some((key, value)) = pair.split_once('=') {
                    query_params.insert(key.to_string(), value.to_string());
                }
            }
        }

        // Remove the request line (already processed)
        request_lines.remove(0);

        // Find the index of the first blank line (i.e., `\r\n`) to separate headers from body
        let blank_line_index = request_lines
            .iter()
            .position(|&line| line == "\r\n")
            .unwrap();

        // at this point i am a genius. I love rust!
        // fixed this shit and extracted the content from this.
        // Split the lines: everything after blank line is the body
        let body_lines = &mut request_lines.split_off(blank_line_index);
        body_lines.remove(0); // Remove the blank line itself
        let body_content = body_lines.join(""); // Reconstruct body as a single string

        // now let's fix the header

        // Parse headers
        for header_line in request_lines {
            if let Some((key, value)) = header_line.split_once(": ") {
                let clean_value = value.replace("\r\n", "");
                header_map.insert(key.to_string(), clean_value);
            }
        }

        Ok(Self {
            method: http_method.to_string(),
            path: path.to_string(),
            content: body_content,
            headers: header_map,
            query: query_params,
        })
    }
}
