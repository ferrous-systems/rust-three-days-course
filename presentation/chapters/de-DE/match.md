# Control-Flow mit `match`

---

Um die Varianten von Enums zu prüfen, wird match verwendet.

---


```rust
fn main() {
    let args = std::env::args;
    
    match args.at(1) {
        Some(arg) => { println!("Argument übergeben: {}", arg)},
        None => { println!("Kein Argument übergeben") }
    }
}
```

---

```rust
fn main() {
    let maybe_file = std::fs::File::open("Gibt's nicht!");
    
    match maybe_file {
        Ok(f) => { println!("Datei geöffnet! Debug: {:?}", f) },
        Err(e) => { println!("Datei nicht geöffnet! Fehler: {:?}", e) }
    }
}
```
---

Matches müssen alle Varianten abdecken.

---

## ignorieren weiterer Varianten


```rust
fn main() {
    let maybe_file = std::fs::File::open("Gibt's nicht!");
    
    match maybe_file {
        Err(e) => { println!("Datei nicht geöffnet! Fehler: {:?}", e) }
        _ => {}
    }
}
```

---

Results tragen eine besondere Markierung: sie dürfen nicht ignoriert werden!

```rust
fn main() {
    let maybe_file = std::fs::File::open("Gibt's nicht!");
}
```

Lösung: matchen oder weiterreichen.

