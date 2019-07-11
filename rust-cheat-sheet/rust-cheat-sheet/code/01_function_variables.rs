// Declaring a function with type annotated arguments
// and a specified return type.
fn add_them(first: i32, second: i32) -> i32 {
    first + second
}

fn main() {
    //Assigning a variable.
    let a_variable = 1;
    //Reassigning a variable.
    a_variable = 2;
    // A mutable variable
    let mut some_value = 1;
    // An immutable variable with explicit type
    let explicitly_typed: i32 = 1;
    // A function call
    some_value = add_them(some_value, explicitly_typed);
    // Macro, note the !
    println!("{}", some_value);
}
