use example::MyClass;

fn main() {
    unsafe {
        let mut m = MyClass::new(1);
        println!("{:?}", m);
        m.set(2);
        println!("{:?}", m);
    }
}