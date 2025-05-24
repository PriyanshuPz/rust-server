// static status_line = "HTTP/1.1 200 OK";
// static contents = "{'error': 'METHOD NOT SUPPORTED'}";
// static length = contents.len();
// static response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}");
// "{'message': 'THIS IS THE DEFAULT RESPONSE'}";

pub static DEFAULT_RESPONSE_U8: &[u8] = "HTTP/1.1 200 OK\r\nContent-Length: 43\r\nContent-Type: application/json\r\n\r\n{\"message\": \"THIS IS THE DEFAULT RESPONSE\"}".as_bytes();
pub static DEFAULT_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Length: 43\r\nContent-Type: application/json\r\n\r\n{\"message\": \"THIS IS THE DEFAULT RESPONSE\"}";
