fn main() {
    let mut args = std::env::args();

    match args.nth(1) {
        Some(arg) => { println!("Argument übergeben: {}", arg)},
        None => { println!("Kein Argument übergeben") }
    }
}