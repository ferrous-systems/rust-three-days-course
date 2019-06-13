fn main() {
    // creating a new empty mutable vector for i32 values
    let mut empty_vector: Vec<i32> = Vec::new();

    // creating a vector with values
    let filled_vector = vec![1, 2, 3];

    // updateing a vector
    empty_vector.push(5);
    empty_vector.push(6);

    // iterating over the values a vector
    for i in &filled_vector {
        println!("{}", i);
    }

    // iterating over mutable references of each element
    // in a mutable vector.
    for i in &mut empty_vector {
        *i += 50;
    }
}
