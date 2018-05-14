1. create a library project called `redisish`
2. Implement the following function

```rust
// appropriate data structures

pub fn parse(input: &str) -> Result<Command, Error> {
    ///...
}
```

3. Test as appropriate

## The protocol

The protocol has two commands:

"PUBLISH \<message\>\n"

"RETRIEVE\n"

These commands should be parsed into the `Command` value,
`Error`s (like empty input, and unknown verb, etc.) should be
appropriately returned.

A missing newline is an error, everything after the first newline can be ignored.

Empty messages are allowed.

## Hints

Read the docs for `str` (primitive), especially `splitn` and `trim`.
