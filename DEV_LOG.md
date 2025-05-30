# A Masochist's Journey to Building an HTTP Server from Scratch

I've always been curious about Rust. The buzz around its memory safety, zero-cost abstractions, and fearsome compiler made me both intrigued and intimidated. Instead of starting with tutorials or relying on heavy frameworks, I decided to try something different — build a tiny server from scratch.

Ah yes, because when normal people want to learn a new programming language, they start with "Hello World." But me? I thought, "Let's reinvent the wheel, but make it square, and also let's use only stone tools while we're at it."

It might sound boring, or even like a waste of time (and yes, at times it felt like that), but I ended up learning a lot through the process. To take things up a notch, I decided to do it using only the standard library — no external crates.

The challenge? Build a simple REST API server for a to-do app, with just the bare minimum. No `tokio`, no `hyper`, no `serde`. Just Rust's `std` library. At least, that was the plan — until things got overwhelming enough to reconsider. 😉

Spoiler: By "overwhelming," I mean the moment I realized parsing JSON without `serde` is like trying to eat soup with a fork — technically possible but an exercise in futility.

## Setup and Getting Started

As a professional beginner, I started by watching a crash course on Rust before diving in. Trust me, I knew nothing about Rust.

"Professional beginner" is my politically correct way of saying "I have impostor syndrome but with extra steps." This way, when I inevitably mess up, I can say, "Well, what did you expect? I'm professionally bad at this!"

After watching a simple crash course and learning how Rust works and what its syntax looks like, I realized for the first time — it's not going to be simple.

Rust's learning curve isn't a curve at all. It's a brick wall with spikes on it, and the Rust compiler stands atop it, looking down at you with a mixture of pity and disappointment, like a cat watching you try to open a can of tuna with your bare hands.

---

# DAY 1: In Which I Convince Myself I Know Rust

Ignoring the existential dread that comes with learning a new systems programming language, I installed Rust on WSL with a simple one-liner from Rust's official site. Then created a fresh project and started printing "hello world" so I could motivate myself that I _do_ know Rust.

```bash
cargo new rust-server
cd rust-server
cargo run
```

Five minutes in, and I'm basically Jon Skeet of the Rust world. Put that on my LinkedIn.

After wasting some time patting myself on the back for successfully printing text to a console (a true technological breakthrough), I opened Rust docs for the standard library and found `TcpListener`. I quickly created a server using it:

```rust
let listener = TcpListener::bind(("127.0.0.1", 5000)).unwrap();
```

It was simple — just gave the address and unwrapped any error.

It is **not recommended to unwrap** any function that can give an error in production, because if an error occurs, it will panic and stop the program. Since this is **not production code**, I can do it.

In fact, I'll be sprinkling `unwrap()`s like confetti throughout this project. My error handling strategy: "If it breaks, I'll deal with it... never." This is what we call "confidence" in the software development industry.

After creating the listener, we need to listen for incoming requests. Here's how:

```rust
for stream in listener.incoming() {
	let stream = stream.unwrap();
	handle_connection(stream);
}
```

Very basic, single-threaded server that will listen for one incoming request at a time.

For now, I don't think I will add multithreading (I was wrong. I did that).

My famous last words. Right up there with "How hard could it be?" and "I'm sure we don't need unit tests for this."

With classic beginner energy, I unwrapped the stream and called `handle_connection` — a function I defined like this:

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_read = BufReader::new(&mut stream);
    
    for line in buf_read.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break; // End of headers
        }
        println!("Received: {}", line);
    }
}
```

Believe me or not, printing this took serious time.

By "serious time," I mean I spent 30 minutes debugging why my buffer reader wouldn't print anything, only to realize I had forgotten to actually send a request to the server. Yes, I'm that developer who checks if the monitor is plugged in after calling IT support.

It's very basic — create a buffer reader and loop through each line, printing it to the console.

At the end of the day, you'll see something like this:

> Here, `mut` is used with `&`.  
> In Rust, `mut` is for making a variable mutable, and `&` is for referencing a variable instead of copying it.  
> So, `&mut stream` is a mutable reference. If you try to access or modify the stream again outside this function, you'll get an error (ownership rules!).

Did I mention Rust has ownership rules? It's like that friend who won't lend you anything without making you sign a contract in blood. "You can borrow my stream, but I need it back EXACTLY as you found it, or the compiler will hunt you down."

```bash
Received: GET / HTTP/1.1
Received: Host: localhost:5000
Received: User-Agent: curl/8.5.0
Received: Accept: */*
```

You're seeing how HTTP works. This is the [protocol](https://www.rfc-editor.org/rfc/rfc1945#section-3).

Ah, the raw HTTP request in all its glory. It's like seeing how sausage is made—oddly satisfying but makes you appreciate the abstractions that normally hide it from you.

### In the **first line**, we have:

- **Method** – what action to perform (like GET/POST). Case-sensitive. [RFC](https://www.rfc-editor.org/rfc/rfc1945#section-5.1.1)
    
- **Request-URI** – the path. [Section 3.2](https://www.rfc-editor.org/rfc/rfc1945#section-3.2)
    
- **Version** – the HTTP version. [RFC](https://www.rfc-editor.org/rfc/rfc1945#section-3.1)
    

The rest are headers: Host, User-Agent, etc.

Yes, I'm actually linking to the RFCs, which makes me feel very important. Nothing says "I'm a serious developer" like citing RFC documents that nobody is actually going to read. But they're there, just in case you want to fall asleep really quickly.

If we send a POST request, we also get the body:

```bash
Received: POST / HTTP/1.1
Received: Host: localhost:5000
Received: Content-Type: application/json
Received: User-Agent: curl/8.5.0
Received: Content-Length: 21
Received: 
Received: {"hello": "John Doe"}
```

For now, we need to extract:

- Method
    
- Path
    
- Body
    
- Headers
    
- Query parameters
    

Rest is useless (for now).  
Extracting it from this plain stream isn't easy — it's just text with no types.  
It'll be fun to do (it won't be).

Spoiler: When a developer says something will be "fun," they mean it will be a soul-crushing exercise that makes them question their career choices. But hey, at least there's coffee.

---

### Sending the Response

Right now, the request just hangs because we're not sending anything back.

It's like when you text someone a question and they read it but don't respond. Rude, right? Our server is currently that person.

Let's write a manual response:

```rust
let status_line = "HTTP/1.1 200 OK";
let contents = "{'hello': 'World'}";
let length = contents.len();
let response = format!(
    "{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}"
);
stream.write(response.as_bytes()).unwrap();
```

Boom. Response sent.

When I typed "Boom," I felt like I just launched a SpaceX rocket, but in reality, I just sent 18 bytes over localhost. Let's keep our achievements in perspective.

```bash
HTTP/1.1 200 OK
Content-Length: 18
Content-Type: application/json

{'hello': 'World'}
```

### That's It for Day 1

That's all I did. Simple stuff — but I was happy. This bit of work taught me:

- How to spin up a TCP server in Rust with only the standard library.
    
- How HTTP requests actually look on the wire.
    
- How to send a raw HTTP response manually.
    

More to come in Day 2.

I ended the day feeling like I'd accomplished something significant, when in reality all I'd done was reinvent technology from the 1990s. But you know what? Sometimes you need to go back to understand how to move forward. At least that's what I tell myself to feel better about spending an entire day writing what most frameworks do in three lines of code.

---

# DAY 2: HTTP Parsing Hell, or "Why Don't I Just Use a Framework Again?"

Today's mission? Turn a chaotic stream of bytes into a neat little `Request` struct that we can actually work with. Think of it like decoding alien signals, except it's HTTP and slightly less cool. The goal is to be able to write code like this:
```rust
fn handle_connection(mut stream: TcpStream) {
    let req = Request::new(&stream);
    println!("REQUEST METHOD: {:?}", req.method);
}
```

Yep, just calling `Request::new(&stream)` should give us all the sweet, sweet info — method, path, headers, query, body. But before that happens, we need to teach our program how to read the stream and not choke.

If you're wondering "why don't I just use a library for this?" — congratulations, you're smarter than me. But we're committed to the bit now, so let's keep reinventing this particular wheel until it's almost round-ish.

So, To implement it let's create a struct named `Request` for this I will create a new file `request/mod.rs`. And create `lib.rs` file in root. Inside `lib.rs` I will register `request/mod.rs` like this:

```rust
pub mod request;
```
This will register this module, add you will get autocompletion if using VS Code.

Ah yes, autocomplete, the thing that writes 90% of my code for me. Let's be honest, without IntelliSense, most of us would be reduced to grunting and throwing rocks at our computers.

Now, Inside `request/mod.rs` let us define the struct.
```rust
#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub content: String,
}
```

This request struct will have these stored stuff we talked in Day 1. Also, added `Debug` macro for debugging it is optional. For this struct we need to define only one function named `new` which will intake the `TCPStream` reference and in result gives response or error string. 
Filename: `request/mod.rs`

```rust
// ...request struct code
impl Request {
	pub fn new(mut stream: &TcpStream) -> Result<Self, String> {
	 // processing code...
	}
}
```

Like this we can create a function which is public so can be accessed by anywhere taking `TCPStream` and resulting request. 

I've gone for the `Result<Self, String>` return type because nothing says "I'm a Rust developer" quite like wrapping everything in `Result`. It's like putting your code in bubble wrap — sure, it's harder to use, but at least it won't break if you drop it.

Our **first step** is to grab all the text from the buffer and store it. 
we need to read the bytes from the stream. For that, we define a couple of variables:
```rust
const MESSAGE_SIZE: usize = 1024;
let mut received: Vec<u8> = vec![]; // this will store full request
let mut rx_bytes = [0u8; MESSAGE_SIZE]; // temporary buffer
```

**Q: Why use a fixed-size buffer like `rx_bytes` instead of reading directly into `received`?**

A: Because `stream.read()` wants a fixed-size buffer to read into. I spent way too much time figuring that out. Don't be like me. Just use the buffer.

True story: I spent about an hour trying various combinations of `.read_to_end()`, `.read_to_string()`, and custom buffer strategies before realizing that TCP streams don't have a clear "end" until they're closed. The documentation mentions this, but who reads documentation, am I right? (Please read documentation.)

Now the scary part: the loop.
```rust
loop {
    let bytes_read = stream.read(&mut rx_bytes);
    match bytes_read {...}
}
```

This loop keeps reading the stream into `rx_bytes`, and each chunk is pushed into `received`. We stop reading once fewer bytes are read than our buffer size — usually meaning the request has ended.

Also, remember: I'm just sharing my experience. If you copied this loop from Stack Overflow, check it before you run it. 😂

Let's be honest, we're all copying from Stack Overflow. The difference between junior and senior developers is that seniors know which parts to modify after copying.

```rust
match bytes_read {
		Ok(bytes) => {
		received.extend_from_slice(&rx_bytes[..bytes]);
		if bytes < MESSAGE_SIZE { break; }
		}
		Err(err) => {
			println!("error: {:#?}", err);
		}
	}
```

We are handling two condition if result is `OK` and on `Err`. For error, we are doing basic console logging. And For `Ok`. We are getting a variable named `bytes` which is a `usize` integer that is the **number of bytes that were successfully read**. Only need to convert it in string and then extract the information.

The error handling here is what I call "log and pray" — we print the error and hope someone is watching the console. This is state-of-the-art error handling for weekend projects.

Now, **second step** is to convert this `received` vector in string:
```rust
let request_text = String::from_utf8(received).unwrap();
```

This allows us to convert it into string. Now, will split this `request_text` with `newline` character. And create some `HashMap` variables to store headers and query parameters.
```rust
let mut request_lines: Vec<&str> = request_text.split_inclusive('\n').collect();

// Store headers and query parameters
let mut header_map: HashMap<String, String> = HashMap::new();
let mut query_params: HashMap<String, String> = HashMap::new();
```

Ah, the classic "split and collect" pattern — the programming equivalent of opening a UPS package with safety scissors. It works, but there's a lot of awkward maneuvering involved.

Now, let's start extracting stuff. Foremost the easy one http method which is in first time.
```rust
// First line of HTTP request, e.g., "GET /path?x=1 HTTP/1.1"
let request_line = request_lines[0];
let mut parts = request_line.split_ascii_whitespace();
// Extract HTTP method and full path
let http_method = parts.next().unwrap();
let full_path = parts.next().unwrap();
```

Here, We have taken first line from the `request_lines` vector we created. Then, split that with whitespace and assigned first part as the `http_method` and second to `full_path`.
`full_path` will have our path name and query parameters that we will extract by separating this with `?`.

Notice those beautiful `.unwrap()` calls? That's me saying "I trust this HTTP request to be perfectly formatted." Which is like trusting a toddler with cake — it's gonna get messy.

```rust
// Separate path and query string (e.g., "/search?q=rust")
let path_and_query: Vec<&str> = full_path.split('?').collect();
let path = path_and_query[0];
```

After separating the first one will always be the path and rest we will handle if the length of the `path_and_query` is greater than 1 which means query params are passed than.
```rust
if path_and_query.len() > 1 {
	let query_string = path_and_query[1..].join("");
	let query_pairs: Vec<&str> = query_string.split("&").collect();
	for pair in query_pairs {
		if let Some((key, value)) = pair.split_once('=') {
			query_params.insert(key.to_string(), value.to_string());
		}
	}
}
```

Further, `query_string` is the string after joining the rest of the stuff excluding the `path`. You can see the spreading `[1..]` This will exclude `1` index keeping rest same.
Afterward, again splitting this with `&` because request might have multiple queries. Then looped `query_pairs` splitting with `=` once and inserting the values in the `HashMap` we created `query_params`.

Every time I write code like this, a framework developer somewhere sheds a single tear. We're essentially manually parsing query parameters like it's 1999. Next, I'll be setting up my Geocities page and adding a visitor counter.

Fun fact till now we just manipulated only first line of the protocol. So, let just remove that first line from the `request_lines` so, it will no conflict with upcoming stuff.
```rust
// Remove the request line (already processed)
request_lines.remove(0);
```

Now, **third step** is to extract body and header so to do that we need to separate them by `\r\n` in to pieces first will be the headers part and rest is body. Let's get the index of this blank line:
```rust
let blank_line_index = request_lines.iter().position(|&line| line == "\r\n").unwrap();
```

After getting the index just spilt off body out
```rust
let body_lines = &mut request_lines.split_off(blank_line_index);
body_lines.remove(0); // Remove the blank line itself
let body_content = body_lines.join(""); // Reconstruct body as a single string
```

Here, `.split_off()` will give the part including the index provided so, removed that in the second line with `.remove()`. You can observe that I have specified `&mut` in variable `body_lines` this will give a mutable reference of the `body_lines` that is why we are able to remove first line. While implementing I realized for the second time — Rust is not for noobs.

"Rust is not for noobs" should be their official slogan. The Rust compiler is like that strict teacher who makes you show all your work and won't let you get away with anything. "You're using a mutable reference here, but did you declare ownership rights? I THINK NOT."

Again stitching body content in a string that can be parsed by `serde` or custom parser. I am not going that deep for now. If you have a hobby to waste time reading these types of devs log than you can try yourself to build one parser or pin me to do. (sarcasm)

Let's be real: if you're still reading this, you either have an unhealthy fascination with HTTP parsing or you're my mom (hi mom!). Either way, I'm concerned about your hobbies.

**Last step**, the headers! Well it is similar to query params with for loop, if you are following along then you can, you do this by your own buddy. Come-on, remember what you read in starting.

Oh, you actually want me to explain the headers parsing too? Fine, here's roughly what it would look like:

```rust
for header_line in &request_lines {
    if header_line.trim().is_empty() {
        continue;
    }
    if let Some((key, value)) = header_line.split_once(": ") {
        header_map.insert(key.to_string(), value.trim().to_string());
    }
}
```

It's basically the same idea as parsing query parameters, except with a different delimiter. If you're having trouble with this, maybe programming isn't for you. (I'm kidding, it took me three tries to get this right.)

And… that's it! That's Day 2.

You can now instantiate a `Request` from a raw TCP stream and access `method`, `path`, `query`, `headers`, and `body`.

If you read all this and are still here, maybe you have a strange hobby of reading dev logs like these. Either way, thanks for coming along the ride. 🤝

Day 2 complete, and all we've done is parse HTTP requests manually, something that libraries have been doing reliably for decades. But hey, we've learned how HTTP works at a fundamental level, which is... useful knowledge to have, I guess? Let's pretend this wasn't a gigantic detour into "things you'll never need to implement yourself" territory.

---

# DAY 3: Responding With Style (Or At Least Valid HTTP)

So far we have a rust server where we can handle the request like this:
```rust
fn handle_connection(mut stream: TcpStream) {
    let req = Request::new(&stream);
    println!("REQUEST METHOD: {:?}", req.method);
}
```

Now, we need a way to send response in a good manner currently we are doing it manually, and it is boring too.
So, similar to `Request` we will create a new struct for `Response` than will create some functions to send the response back.

Boring is programmer-speak for "I've typed this same boilerplate 17 times and I'm starting to wonder if computers were a mistake."

This is the manual code:
```rust
let status_line = "HTTP/1.1 200 OK";
let contents = "{'hello': 'World'}";
let length = contents.len();
let response = format!(
    "{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}"
);
stream.write(response.as_bytes()).unwrap();
```

What we can do is create a function to intake status code, content and optionally headers then return that response. Also, to write this response we can create a function to resolve. In practical let us code to handle some endpoints:

Look at all those string concatenations and hard-coded header values. This code is begging to be abstracted away. It's like watching someone solve a Rubik's cube by peeling off the stickers — it works, but it hurts my soul.

```rust
fn handle_connection(mut stream: TcpStream) {
	let req = Request::new(&stream);
	let res = match req {
		Ok(req) => {
			let res = match req.path.as_str() {
				"/hello" => {
                    let default_name = "World".to_string();
                    let name: &String = req.query.get("name").unwrap_or_else(|| &default_name);
                    let payload = format!("{{\"message\": \"Hello, {}!\"}}", name);
                    Response::json(200, &payload, None)
                }
                &_ => {
                    let payload = "{{\"message\": \"Invalid Path\"}}";
                    Response::json(400, &payload, None)
                }
            };
            Response::resolve(&res)
		}
		Err(s) => {
			let payload = format!("{{\"message\": \"{}\"}}", s);
            let res = Response::json(500, &payload, None);
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
```

In this code snippet, we are matching the request path and conditionally executing required code. Here `Response::json()` and `Response::resolve()` function today, we will implement these are helping us in sending a clear formatted response. 

This is what I call "nested match statement hell." It's like Russian nesting dolls, except each doll contains a slightly different way of saying "something went wrong."

Let's start by creating a new file `response/mod.rs` and add this in `lib.rs` file like we did for `Request`.

In `response/mod.rs` we will create a new struct named `Response` containing status code, body and headers. Then will implement two functions we decided `::json` and `::resolve`.
```rust
pub struct Response {
    status_text: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Response {
	pub fn json(status: u16, body: &str, headers: Option<Vec<(String, String)>>) -> Self {
	 // conversion to response code
	}

	pub fn resolve(response: &Response) -> String {
		// response to string code
	}
}
```

You know you're writing Rust when half your code is just setting up structures to make the other half cleaner. It's like spending 3 hours organizing your desk before starting a 10-minute task. But hey, that's why we love it!

These functions are going to be very simple firstly `::json()` will calculate the body length and initialize some predefined headers
```rust
let content_length = body.len();
let pre_dermined_headers = vec![
	("Content-Type".to_string(), "application/json".to_string()),
	("Content-Length".to_string(), content_length.to_string()),
];
let headers = headers.unwrap_or_else(|| vec![]);
```

We are using content type to be `json` as this function is only for `json` if you want you a creation for HTML you can do similarly.
Now, will creating a variable for status text which is a string like for 200 `200 OK`, 201 `201 Created`, and so on.
```rust
let status_text = match status {
	200 => "200 OK".to_string(),
	400 => "400 Bad Request".to_string(),
	500 => "500 Internal Server Error".to_string(),
	_ => format!("{} Unknown", status),
```

I love how we're handling only three HTTP status codes here. It's like opening a restaurant that only serves breakfast, lunch, and dinner, but nothing in between. Want a mid-afternoon snack? Sorry, "418 I'm a teapot" is not on the menu.

To keep it very simple I've added only three codes, you can extend it if you want.
Lastly, return this response struct and our `::json()` function is done.
```rust
Self {
	status_text,
	headers: [pre_dermined_headers, headers].concat(),
	body: body.to_string(),
}
```

Now, only `::resolve()` function is remaining which is a simple logic to stitch all these stuff together and create a string to send
```rust
let mut response_str = format!("HTTP/1.1 {}\r\n", response.status_text);

for (key, value) in &response.headers {
	response_str.push_str(&format!("{}: {}\r\n", key, value));
}

response_str.push_str("\r\n");
response_str.push_str(&response.body);

response_str
```

This `resolve` function is basically doing the same string concatenation we were trying to avoid, but now it's encapsulated so we can pretend we don't see it. This is what we call "abstraction" in the industry — putting ugly code in a box and giving it a nice name.

So far, now we have somewhat usable rust server that intake request and throw response in little good manner.

It was day 3 of mine, And I think we can create simple to-do API with this. The only remaining is to add multi-threading that is the last chapter.

"Somewhat usable" is developer-speak for "it works on my machine under perfect conditions, but I wouldn't trust it with actual users." But we're making progress! We've graduated from "completely unusable" to "somewhat usable" in just three days.

---

# DAY 4: Multithreading Madness, or "How I Learned to Stop Worrying and Love the Thread Pool"

Today marks the final day of this dev log. It's been an exciting, fulfilling journey — a low-level, hands-on exploration of building an HTTP server in Rust using nothing but the standard library. From parsing raw TCP streams to crafting custom `Request` and `Response` types, and now finally implementing a thread pool — this was more than a weekend project. It was a masterclass in systems programming, Rust safety, and architectural thinking.

"Exciting" and "fulfilling" are the words I use in public. In private, it was more like "frustrating" and "why did I choose this again?" But now that it's over, I can pretend it was a smooth, enjoyable experience. That's how memory works!

Let's dive into Day 4, which was all about making our server concurrent and robust. But as with all systems work, the deeper story isn't just _what_ I built — it's _why_ I built it that way.

Or more accurately, _why_ I thought reinventing thread pooling was a good use of my time when perfectly good libraries exist. But here we are.

## Motivation: From Sequential to Concurrent

Initially, our server could handle one request at a time. If a user hit `/hello`, and inside that handler we added something like:

```rust
std::thread::sleep(std::time::Duration::from_secs(5));
```

The whole server would freeze for 5 seconds. No other request could get through. That's obviously not how real-world web servers behave — not even the simplest ones.

It's like having only one cashier at a grocery store who also has to bag your items, fetch missing products, and check inventory. You'd have a line out the door faster than you can say "unexpected item in bagging area."

The natural next step: let's add multithreading.

I began with a naive approach, just like any curious Rustacean learning the ropes would.

## First Stop: Spawning a New Thread per Request

Here's the naive, simple version:

```rust
for stream in listener.incoming() {
    let stream = stream.unwrap();
    std::thread::spawn(|| {
        handle_connection(stream);
    });
}
```

This actually works well for testing — the server no longer blocks on a single request. We can add delay inside the handler, and other clients won't wait.

But here's the caveat: we're spawning a new thread for every single connection.

What happens if someone sends us 1,000 requests per second? We spawn 1,000 threads per second. The system dies fast. This is a textbook Denial of Service vector.

It's like hiring a new employee for each customer that walks into your store. By the end of the day, you'd have 500 employees standing around, payroll would be impossible, and your store would be too crowded for any actual customers.

So I pivoted to a better design.

## The Thread Pool Model

A **Thread Pool** is like a team of waiters at a restaurant. You don't hire a new waiter for every customer. You just hand tasks to whoever's available.

Or think of it as a group of interns eagerly waiting to do your grunt work. "You there, handle this HTTP request! You, parse this JSON! The rest of you, stand by for more tedious tasks!"

Technically, a thread pool keeps a fixed number of threads alive. Each thread waits for a job. When a job (like an incoming HTTP request) arrives, it's pushed into a shared queue. The next available thread grabs it and handles it.

This avoids overloading the OS scheduler with thousands of threads while still enabling concurrency.

In Rust, building a thread pool from scratch meant juggling:

- **Threads** (from the standard library),
    
- **Channels** (`std::sync::mpsc`) for sending jobs,
    
- **Mutexes** and **Arcs** for safe, shared ownership of the job receiver.
    

Let's walk through the code.

By "let's walk through the code," I mean "brace yourself for a cavalcade of Rust's most intimidating concurrency primitives." If you've never seen an `Arc<Mutex<mpsc::Receiver<Box<dyn FnOnce() + Send + 'static>>>>` before, well... you're about to.

## 📦 The ThreadPool Code: Building a Worker Army

We created a new module: `lib.rs` (or `thread_pool.rs` if separated), which defines `ThreadPool`.

### `ThreadPool` Struct

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
```

It holds:

- `workers`: a vector of threads.
    
- `sender`: a channel to send jobs into the queue.
    

We define `Job` like this:

```rust
type Job = Box<dyn FnOnce() + Send + 'static>;
```

A job is just a closure you can run once. It must be thread-safe (`Send`) and have a static lifetime (since threads can outlive their scopes).

Look at that type alias. If you can read that and immediately understand what it means, congratulations! You've achieved Rust enlightenment. For the rest of us mortals, it's a boxed closure that can be sent between threads and lives for the entire program.

### Creating the Pool

```rust
pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
        workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool {
        workers,
        sender: Some(sender),
    }
}
```

- We create a sender/receiver channel pair.
    
- The receiver is wrapped in `Arc<Mutex<>>`, so it can be safely shared between threads.
    
- Each worker is spawned with a cloned handle to the shared receiver.
    

That `Arc<Mutex<>>` wrapper is the Rust equivalent of putting something in a special box with a lock on it, then making photocopies of the box. Everyone gets their own copy of the box, but there's still only one thing inside, and only one person can unlock it at a time.

### The Worker: Thread That Never Sleeps

```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```

Each worker has an ID and a thread handle. Inside `Worker::new`:

```rust
fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop {
        let message = receiver.lock().unwrap().recv();

        match message {
            Ok(job) => {
                job(); // Execute the closure!
            }
            Err(_) => {
                println!("Worker {id} disconnected; shutting down.");
                break;
            }
        }
    });

    Worker {
        id,
        thread: Some(thread),
    }
}
```

Each thread loops forever, waiting for a job via the channel. If it gets one, it runs it. If the channel closes, it breaks the loop and shuts down.

You know what this reminds me of? Those poor souls at the fast-food drive-thru window, eternally waiting for the next customer to pull up, ready to take their order until the restaurant closes (the channel shuts down). Except in our case, the orders are HTTP requests, and the food is... JSON responses? This metaphor might be breaking down.

### Submitting Work

Back in `main.rs`, we replace:

```rust
handle_connection(stream);
```

with:

```rust
let pool = ThreadPool::new(4);
for stream in listener.incoming() {
    let stream = stream.unwrap();
    pool.execute(|| {
        handle_connection(stream);
    });
}
```

And define `execute` as:

```rust
pub fn execute<F>(&self, f: F)
where
    F: FnOnce() + Send + 'static,
{
    let job = Box::new(f);
    self.sender.as_ref().unwrap().send(job).unwrap();
}
```

This sends the job to the shared channel, where a worker picks it up.

Notice how we're using generic function parameters, type bounds, and lifetime specifications just to... send a function to another thread. Rust makes simple things complex so that complex things can be simple. It's like doing extensive paperwork to get a library card, but once you have it, you can borrow any book without further hassle.

## Cleaning Up: Implementing Drop for ThreadPool

When our server exits, we don't want zombie threads. So we implement `Drop`:

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // Closes the sending side of the channel

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

This cleanly shuts down each worker.

The `Drop` trait is Rust's version of a cleanup crew. It's like telling your party guests "Please take your belongings with you when you leave" instead of finding random shoes and jackets in your house weeks later. Memory management is just good manners.

## 🧱 The Final Structure of the Server

By the end of Day 4, here's what the server looks like:

- `Request`: Parses method, path, headers, query params, body from raw bytes.
    
- `Response`: Provides `Response::json` and `Response::resolve` to build HTTP responses.
    
- `ThreadPool`: Handles incoming connections in parallel.
    
- `handle_connection`: Contains the actual routing and logic for endpoints like `/hello`.
    

When you step back and look at it, we've essentially recreated tiny, limited versions of:

- `hyper` (HTTP parsing)
- `serde_json` (JSON handling, though barely)
- `tokio` or `rayon` (concurrency)

All to... parse an HTTP request and return "Hello, World!" I'm not sure whether to feel proud or deeply question my life choices.

## 🎉 Final Thoughts

Five days ago, I started with nothing but an idea: can I build a fully working HTTP server using just the Rust standard library?

Five days ago, I also had friends, social interactions, and a healthy relationship with sunlight. I've since traded these for the satisfaction of parsing HTTP headers manually.

Here's what I ended up with:

✅ Parsed raw TCP streams into `Request` structs  
✅ Built a structured `Response` system with content-type headers  
✅ Implemented conditional routing (`/hello`) with query param parsing  
✅ Added a concurrent thread pool to handle multiple connections  
✅ Cleaned up threads gracefully using the `Drop` trait  
✅ Never used a single external crate  
✅ Questioned my sanity approximately 17 times

It wasn't just about learning how to write TCP servers. It was about understanding the internals that most high-level frameworks abstract away.

It's like learning how a car engine works instead of just driving the car. Will this knowledge help me get to the grocery store faster? Absolutely not. But will it make me feel superior when talking to people who just drive their cars without knowing what a carburetor is? You bet.

This server is far from production-ready, but it's clear, readable, and educational. You can easily extend it with features like:

- `POST` form data body parsing
    
- File upload/download
    
- Static file serving
    
- Logging middleware
    
- TLS support (with crates like `native-tls`)
    
- Async I/O using `tokio` (if leaving stdlib constraint)
    
- A will to live (this one might be harder to implement)

But for now, I'm happy.

This journey reminded me that _you don't always need more abstractions_. Sometimes, starting from scratch helps you understand what abstractions are truly worth.

It also reminded me that there's a reason why web frameworks exist, and why nobody in their right mind parses HTTP manually in production. But hey, learning is fun, right?

Thank you, Rust, for making low-level work feel safe and expressive.

And thank you, dear reader, for sticking with me through this unnecessarily detailed account of my descent into HTTP parsing madness.

---

🔚 **End of Dev Log**

💡 If you read this and decide to build your own server from scratch — do it. You'll never look at `actix-web` or `warp` the same way again. You'll look at them with tears of gratitude in your eyes, whispering "thank you for existing" as you add them to your dependencies.

🛠️ Signing off — see you in the next low-level adventure! (Which, if I'm smart, will involve a lot more use of established libraries and a lot less reinventing of wheels.)
