// Inline syntax
fn generic_inline<S: AsRef<str>>(thing: S) -> S {
    thing
}

// Where syntax
fn generic_where<Stringish>(thing: Stringish) -> Stringish 
where Stringish: AsRef<str> {
    thing
}

// Structs too!
struct GenericStruct<A> {
    value: A,
}

fn main() {
    let foo = "foo";
    generic_inline(foo);
    generic_where(foo);
}
