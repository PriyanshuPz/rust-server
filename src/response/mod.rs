pub struct Response {
    status_text: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Response {
    pub fn json(status: u16, body: &str, headers: Option<Vec<(String, String)>>) -> Self {
        let content_length = body.len();
        let pre_dermined_headers = vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Content-Length".to_string(), content_length.to_string()),
        ];

        let headers = headers.unwrap_or_else(|| vec![]);

        let status_text = match status {
            200 => "200 OK".to_string(),
            201 => "201 Created".to_string(),
            202 => "202 Accepted".to_string(),
            204 => "204 No Content".to_string(),
            400 => "400 Bad Request".to_string(),
            401 => "401 Unauthorized".to_string(),
            403 => "403 Forbidden".to_string(),
            404 => "404 Not Found".to_string(),
            405 => "405 Method Not Allowed".to_string(),
            408 => "408 Request Timeout".to_string(),
            429 => "429 Too Many Requests".to_string(),
            500 => "500 Internal Server Error".to_string(),
            501 => "501 Not Implemented".to_string(),
            503 => "503 Service Unavailable".to_string(),
            504 => "504 Gateway Timeout".to_string(),
            505 => "505 HTTP Version Not Supported".to_string(),
            _ => format!("{} Unknown", status),
        };
        Self {
            status_text,
            headers: [pre_dermined_headers, headers].concat(),
            body: body.to_string(),
        }
    }

    pub fn resolve(response: &Response) -> String {
        let mut response_str = format!("HTTP/1.1 {}\r\n", response.status_text);

        for (key, value) in &response.headers {
            response_str.push_str(&format!("{}: {}\r\n", key, value));
        }

        response_str.push_str("\r\n");
        response_str.push_str(&response.body);

        response_str
    }
}
