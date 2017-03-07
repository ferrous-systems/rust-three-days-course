fn main() {
    let number: &mut i32 = 4;
    *number = 10;
    println!("{}", number);
}