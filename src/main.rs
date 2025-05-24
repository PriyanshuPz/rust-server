use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use server::{methods::Methods, request::Request};
use server::{response::Response, ThreadPool};

const SERVER_ADDRESS: &str = "127.0.0.1";
const SERVER_PORT: u16 = 5000;

fn main() {
    let listener = TcpListener::bind((SERVER_ADDRESS, SERVER_PORT)).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(err) => {
                println!("Failed to get stream: {err:?}");
            }
        };
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let req = Request::new(&stream);

    let res = match req {
        Ok(r) => {
            let res = match r.method.as_str() {
                "GET" => Methods::handle_get(r),
                // "POST" => Methods::handle_post(r),
                // "PUT" => Methods::handle_put(r),
                // "DELETE" => Methods::handle_delete(r),
                &_ => Methods::handle_error("Invalid Method"),
            };
            Response::resolve(&res)
        }
        Err(s) => {
            let res = Methods::handle_error(&s);
            Response::resolve(&res)
        }
    };
    match stream.write(res.as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            println!("FAILED DISPATCHED RESPONSE")
        }
    }
}
