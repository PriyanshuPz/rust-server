use crate::{controllers::TodoController, request::Request, response::Response};

pub struct Methods;

impl Methods {
    pub fn handle_get(req: Request) -> Response {
        return match req.path.as_str() {
            "/todos" => TodoController::get_all_todos(),
            _ => {
                let goodbye_payload = format!(
                    "{{\"message\": \"Goodbye, {}!\"}}",
                    req.query.get("name").unwrap_or(&"World".to_string())
                );
                return Response::json(200, &goodbye_payload, None);
            }
        };
    }
    pub fn handle_post(req: Request) -> Response {
        return match req.path.as_str() {
            "/todos" => TodoController::add_todo(req),
            _ => {
                let goodbye_payload = format!(
                    "{{\"message\": \"Goodbye, {}!\"}}",
                    req.query.get("name").unwrap_or(&"World".to_string())
                );
                return Response::json(200, &goodbye_payload, None);
            }
        };
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
