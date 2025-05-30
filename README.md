# Rust HTTP Server from Scratch

This project is a complete implementation of a web server in Rust built from scratch, primarily using only the standard library (`std::net::TcpListener`). It serves as a learning exercise in systems programming and HTTP fundamentals, and includes a functional Todo application example.

> **Note:** This project is no longer under active development but remains available as a learning resource. Contributions are still welcome!

## Features

1. **Core HTTP Server:**
   * Parses and handles HTTP requests using only `std::net::TcpListener`
   * Custom request and response abstractions
   * Thread pool for concurrent connection handling
   * Basic routing system for API endpoints

2. **API Capabilities:**
   * Supports common HTTP methods: GET, POST, PUT, DELETE
   * Query parameter parsing
   * JSON request/response handling

3. **Todo Application Example:**
   * Manages a list of todos, each with an `id`, `title`, and `completed` status
   * Provides CRUD (Create, Read, Update, Delete) operations
   * Persists todo data to a local `data.json` file 

4. **Learning Benefits:**
   * Demonstrates low-level HTTP protocol parsing
   * Shows concurrent programming patterns in Rust
   * Provides insights into web server architecture
   * Illustrates Rust's ownership system and safety features

## Getting Started

```bash
# Clone the repository
git clone https://github.com/PriyanshuPz/rust-server.git
cd rust-server

# Run the server
cargo run
```

Then visit `http://localhost:5000` in your browser or use tools like `curl` to interact with the API.

## Educational Resources

If you're learning from this project, check out these related resources:

- [Rust Book - Building a Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [HTTP/1.1 RFC](https://tools.ietf.org/html/rfc7230)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)

## Contributing

Although this project is no longer under active development, contributions are welcome! Feel free to:

- Fork the repository
- Make your changes
- Submit a pull request

This is a fun project for learning purposes, so creative improvements are encouraged!

## License

MIT

---

*Built with ❤️ and a slightly masochistic desire to understand HTTP from the ground up*
