fn main() {
    // creating a new empty mutable String
    let mut new_string = String::new();

    // creating a String from &str
    let data = "content";
    let other_string = data.to_string();
    // or
    let other_string = String::from("content");

    // Strings are UTF-8 encoded.
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");

    // updating a String
    let mut a_string = String::from("foo");
    a_string.push_str("bar");
}
