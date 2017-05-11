fn main() {
    let value = 2;
    if value % 2 == 0 {
        // ...
    } else if value == 5 {
        // ...
    } else { /* ... */ }
    
    let maybe_value = Some(2);
    if let Some(value) = maybe_value {
        // ...
    } else { /* ... */ }
}