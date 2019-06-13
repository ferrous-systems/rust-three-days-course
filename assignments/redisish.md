#redisish Parser

1. Create a library project called `redisish`.
2. Read the docs for `str` (primitive), especially `splitn` and `trim`. Pay attention to their return type.

3. Implement the following function, so it parses the input according to the rules of the protocol, described below.

```rust
// appropriate data structures

pub fn parse(input: &str) -> Result<Command, Error> {
    ///...
}

```
Use `enums` for the data structures. One for the Commands and one for the different error types. Use `if let` and `match` for control flow.

4. Test as appropriate. (What is appropriate?)


## The protocol

The protocol has two commands:

"PUBLISH \<message\>\n"

"RETRIEVE\n"

These commands should be parsed into the `Command` value,
`Error`s (like empty input, and unknown verb, etc.) should be
appropriately returned.

A missing newline is an error, everything after the first newline can be ignored.

Empty messages are allowed.
