impl Point {
    x: i32,
    y: i32
}

impl Point {
    fn x(&self) -> &i32 {
        &self.x
    }

    fn y(&'a self) -> &'a i32 {
        &self.y
    }
}