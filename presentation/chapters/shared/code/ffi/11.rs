struct Database {
    //...
}

struct DatabaseIterator<'a> {
    database: &'a Database,
    iter: RawIterator
}