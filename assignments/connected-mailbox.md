# The mailbox

Implement a small TCP server, handling requests using the protocol we wrote yesterday.

## 1. Turn you project into a workspace

Read http://doc.crates.io/manifest.html

The workspace should contain your server and the redisish library from yesterday.

```
my_project
|- Cargo.toml
|- redisish
|- connected-mailbox
```

```
$ cat connected-mailbox/Cargo.toml
...
[dependencies]
redisish = { path = "../redisish" }
```

## 2. Accept connections and implement the protocol

1. Every connection just sends one command
2. PUBLISH inserts into a message box
3. RETRIEVE retrieves a message, in a FIFO fashion

Use `.unwrap` for all error handling in the beginning.

Use `std::net::TcpListener` for connecting.

Use `std::collections::VecDeque` for storing.

Read the documentation for the `std::io::Read`, `std::io::Write` traits, especially `read_to_string`.

## 3. Do proper error handling

Implement a `ServerError`, allowing to handle both `io::Error`s and `redisish::Error`s.

## 4. Implement multithreading

Per connection:

1. spawn a thread using `std::thread::spawn`
2. wrap the `VecDeque` in a `std::sync::Mutex` and an `std::sync::Arc` to pass it around


