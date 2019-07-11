struct Empty;

struct WithFields {
    foo: i32,
    bar: Choice,
}

type Explanation = String;

enum Choice {
    Yes,
    No,
    Maybe(Explanation),
}

fn main() {
    // instantiating a struct with one field referring
    // to the variant of an enum.
    let example_struct = WithFields {
        foo: 42,
        bar: Choice::Maybe(String::from("I like dogs"),
    };
}
