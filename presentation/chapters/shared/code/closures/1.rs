fn main() {
    let vec = vec![1,2,3];
    vec.map(|x| x**2).collect::<Vec<_>>();
}