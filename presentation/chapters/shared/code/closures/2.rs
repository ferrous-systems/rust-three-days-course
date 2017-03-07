fn main() {
    let vec = vec![1,2,3];
    let double = |x| { x ** 2 };
    vec.map(double).collect::<Vec<_>>();
}