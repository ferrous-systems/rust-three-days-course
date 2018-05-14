struct Container<'a, T: 'a> {
    inner: &'a T
}