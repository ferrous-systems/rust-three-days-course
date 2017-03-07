fn main() {
    let mut args = std::env::args();

    let value = if let Some(arg) = args.at(1) {
                    arg
                } else {
                    "default!".to_string()
                };
}