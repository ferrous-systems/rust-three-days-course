use std::collections::HashMap;

fn main() {
    // Creating a new hash map with String type keys and i32
    // values.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Creating a hash map from two lists.
    let blue = String::from("Blue");
    let yellow = String::from("Yellow")

    let teams  = vec![blue, yellow];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams
        .iter()
        .zip(initial_scores.iter()).collect();

    // Accessing values stored in the hash map.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Iterating over key value pairs.
    for (key, value) in &scores {
        println!("{}: {}", key, value);

    // Updating the hash hash map.
    // Overwriting existing values:
    scores.insert(String::from("Blue"), 20);
    // Only inserts a value, if the key does not already
    // have one:
    scores.entry(String::from("Yellow")).or_insert(50);

}
