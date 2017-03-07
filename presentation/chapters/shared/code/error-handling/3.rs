fn main() {
    match this_can_fail(false) {
        Ok(val) => println!("Funktioniert: {}", val),
        Err(err) => println!("Gescheitert: {}", err),
    }
}