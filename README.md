## Rust Server from Scratch

This project is a build of a web server in Rust from scratch, primarily using `std::net::TcpListener`. It serves as a learning exercise and now includes a functional Todo application example.

> This project is under active development.

## Features
1.  **Core Web Server:**
    *   Handles HTTP requests using `std::net::TcpListener`.
    *   Supports serving static HTML files.
    *   Basic routing for API endpoints.
2.  **API Capabilities:**
    *   Supports common HTTP methods: GET, POST, PUT, DELETE.
    *   *Contributions to add more methods are welcome!*
3.  **Todo Application Module:**
    *   Manages a list of todos, each with an `id`, `title`, and `completed` status.
    *   Provides CRUD (Create, Read, Update, Delete) operations for todos.
    *   Persists todo data to a local `data.json` file using `serde` for JSON serialization/deserialization.
4.  **Rust Benefits:**
    *   Leverages Rust's memory safety features.
    *   Aims for good performance by utilizing the standard library directly.


# Contribute

As this is a side project, if any of you want to collaborate, feel free to open issues and PRs following general contribution guidelines.