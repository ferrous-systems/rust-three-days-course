// Integers
// All standard sizes, with or without sign
i8, u8
i16, u16
i32, u32
i64, u64
i128, u128

// Architecture-dependent Numbers
isize, usize

// Casts Between Numbers
fn main() {
    let foo = 3_i64;
    let bar = foo as i32;
}

// If the size isn't given, or cannot be inferred, ints
// default to i32.

// Floats
// all standart sizes
f32, f64

// Fixed-size Arrays
let arr: [i32; 4] = [1,2,3,4]

// Dynamic-size Arrays
let arr: &[i32] = &[1,2,3,4];

// The Unit Type
// Expressions without a specific return value return
// the unit type ().

fn main() -> () {
    42;
}
