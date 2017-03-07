struct Point {
    x: i32,
    y: i32
}

trait Distance {
    fn distance(self, other: Self) -> i32;
}

impl Distance for Point {
    // diese Distanzfunktion ist falsch
    fn distance(self, other: Point) -> i32 {
        (self.x - other.x) + (self.y - other.y)
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    println!("{}", p1.distance(p2));
}