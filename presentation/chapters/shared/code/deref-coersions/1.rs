struct Point {
    x: i32,
    y: i32
}

fn main() {
    let boxed_p = Box::new(Point { x: i32, y: i32 });
    println!("{}", boxed_p.x);
}