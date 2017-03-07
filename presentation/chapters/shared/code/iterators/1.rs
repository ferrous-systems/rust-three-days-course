fn main() {
    let items = vec![0, 1, 2];
    let iterator = items.iter();
    assert_eq!(iterator.next(), Some(0));
    assert_eq!(iterator.next(), Some(1));
    assert_eq!(iterator.next(), Some(1));
    assert_eq!(iterator.next(), None);
}