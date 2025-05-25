pub mod todo;
pub use crate::controllers::todo::Todos;
use crate::{request::Request, response::Response};
use serde_json::json;
use todo::PersistTodos;

pub struct TodoController;

impl TodoController {
    pub fn get_all_todos() -> Response {
        let todos = Todos::from_file().get_all_todos();
        let all_todos = json!({
            "todos": todos
        });

        Response::json(200, &serde_json::to_string(&all_todos).unwrap(), None)
    }

    pub fn add_todo(req: Request) -> Response {
        let mut todos = Todos::from_file();

        #[derive(serde::Deserialize)]
        struct Payload {
            content: String,
        }

        println!("Received request: {:?}", req.content);

        let payload: Payload = match serde_json::from_str(&req.content) {
            Ok(todo) => todo,
            Err(_) => {
                return Response::json(400, "{\"error\": \"Invalid JSON format\"}", None);
            }
        };
        let todo = todo::Todo {
            id: todos.get_all_todos().len() as u32 + 1,
            title: payload.content,
            completed: false,
        };
        todos.add_todo(todo);
        Response::json(
            201,
            &serde_json::to_string(&todos.get_all_todos()).unwrap(),
            None,
        )
    }
}
