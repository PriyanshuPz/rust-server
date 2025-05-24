use crate::{request::Request, response::Response};

pub struct Methods;

impl Methods {
    pub fn handle_get(req: Request) -> Response {
        let payload = format!(
            "{{\"message\": \"GET request received for path: {}\"}}",
            req.path
        );
        Response::json(200, &payload, None)
    }
    pub fn handle_post(req: Request) -> Response {
        let payload = format!(
            "{{\"message\": \"POST request received for path: {}\"}}",
            req.path
        );
        Response::json(200, &payload, None)
    }
    pub fn handle_put(req: Request) -> Response {
        let payload = format!(
            "{{\"message\": \"PUT request received for path: {}\"}}",
            req.path
        );
        Response::json(200, &payload, None)
    }
    pub fn handle_delete(req: Request) -> Response {
        let payload = format!(
            "{{\"message\": \"DELETE request received for path: {}\"}}",
            req.path
        );
        Response::json(200, &payload, None)
    }
    pub fn handle_error(error: &str) -> Response {
        let payload = format!("{{\"message\": \"{}\"}}", error);

        Response::json(200, &payload, None)
    }

    pub fn to_string(response: &Response) -> String {
        Response::resolve(response)
    }
}
