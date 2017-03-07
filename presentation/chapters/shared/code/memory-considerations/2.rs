struct Byte {
    field: u8
}

fn main() {
    use std::mem;

    println!("{}", mem::size_of::<One>());

    println!("{}", mem::size_of::<[One; 2]>());
}