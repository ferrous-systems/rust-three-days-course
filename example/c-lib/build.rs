extern crate cheddar;

fn main() {
    cheddar::Cheddar::new().expect("could not read manifest")
        .run_build("include/point.h");
}
