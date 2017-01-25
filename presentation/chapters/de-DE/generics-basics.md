# Simple Generics

---

Generics sind fundamental für Rust.

---

# Generische Structs

```rust
struct Point<Precision> {
    x: Precision,
    y: Precision
}

fn main() {
    let point = Point { x: 1u32, y: 2 };
}

---

# Generische Enums

```rust
enum Option<T> {
    Some(T),
    None
}
```