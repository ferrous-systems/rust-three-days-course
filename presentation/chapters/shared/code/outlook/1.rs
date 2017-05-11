pub trait Extend<A, T: IntoIterator<Item=A>> {
    fn extend(&mut self, iterable: T);
}

// The generic implementation
impl<A, T> Extend<A, T> for Vec<A> where T: IntoIterator<Item=A> {
    // the `default` qualifier allows this method to be specialized
    default fn extend(&mut self, iterable: T) {}
}

// A specialized implementation for slices
impl<'a, A> Extend<A, &'a [A]> for Vec<A> {
    fn extend(&mut self, iterable: &'a [A]) {}
}