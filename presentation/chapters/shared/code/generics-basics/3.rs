enum Option<T> {
    Some(T),
    None
}

fn main() {
    let mut args = std::env::args;
    println!("{:?} {:?}", args.nth(0), args.nth(0))
}