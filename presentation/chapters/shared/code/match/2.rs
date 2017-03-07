fn main() {
    let mut args = std::env::args();

    if let Some(arg) = args.nth(1) {
       println!("Argument übergeben: {}", arg);
    }
}