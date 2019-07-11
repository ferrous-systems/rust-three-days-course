fn main() {
    // Creating a new empty mutable vector for i32 values.
    let mut empty_vector: Vec<i32> = Vec::new();

    // Creating a vector with values.
    let filled_vector = vec![1, 2, 3];

    // Updateing a vector.
    empty_vector.push(5);
    empty_vector.push(6);

    // Iterating over the values a vector.
    for i in &filled_vector {
        println!("{}", i);
    }

    // Iterating over mutable references of each element
    // in a mutable vector.
    for i in &mut empty_vector {
        *i += 50;
    }
}
