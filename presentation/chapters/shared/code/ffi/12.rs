use std::marker::PhantomData;

struct Database {
    //...
}

struct DatabaseIterator<'a> {
    database: PhantomData<&'a Database>,
    iter: RawIterator
}