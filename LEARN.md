## 👋 Welcome to the Learning Guide

This project is a journey — not just a codebase. The goal? To understand what really goes on under the hood when you hit enter on a URL. No frameworks, no shortcuts. Just Rust, sockets, and the raw guts of HTTP.

Whether you're here to learn Rust, dig into low-level networking, or just satisfy your masochistic curiosity — welcome!

---

## 🧠 What You’ll Learn

* How TCP and HTTP work together
* How to parse raw request data into usable structs
* How to build a basic HTTP server from scratch
* How threading, concurrency, and request handling works
* Why abstraction exists — and when to break it for fun

---

## 📚 Suggested Reading Order

1. `main.rs` – Entry point and how the server listens for connections.
2. `request.rs` – Raw TCP stream → structured HTTP request.
3. `response.rs` – Building responses manually (status codes, headers, body).
4. `methods.rs` – Routing and HTTP method handling.
5. `controllers/` – Business logic separated out.
6. `lib.rs` – Our homegrown thread pool.
7. `constants.rs` – All our reused values in one place.

Check out the [DEV Log Post](https://dev.to/priyanshuverma/a-masochists-journey-to-building-an-http-server-from-scratch-1272) for the full story.

---

## ⚠️ Things to Keep in Mind

* This is for learning, not production.
* You *will* make mistakes — that’s the point.
* Use tools like `cargo fmt`, `clippy`, and Wireshark for extra learning.
* Don't be afraid to go one layer deeper. That's where the magic is.

---

## 🎯 Suggestions to Try Next

* Add support for more HTTP methods (`POST`, `PUT`, etc.)
* Implement a static file server
* Try chunked transfer encoding
* Add logging or a basic CLI interface
* Replace `std::net` with `tokio` or go async

---

## ❤️ Why This Exists

Because sometimes, you just want to break everything down and build it back up. Not for speed. Not for shipping. Just for understanding. And if you’re like me, you’ll walk away not just with better skills — but a deeper respect for the tools we use every day.

---

Feel free to fork, break, improve, or just stare in confusion.

Let’s keep learning.

— Priyanshu 🧠💻
