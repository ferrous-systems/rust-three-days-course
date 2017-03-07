trait Double : Into<i64> {
    fn double(self) -> i64 {
        let val: i64 = self.into();
        val * 2
    }
}

impl<T> Double for T where T: Into<i64> { }

fn main() {
    21.double();
}