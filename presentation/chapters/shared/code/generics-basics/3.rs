enum Option<T> {
    Some(T),
    None
}

fn main() {
    let args = std::os::args;
    println!("{:?} {:?}", args.at(0), args.at(1))
}