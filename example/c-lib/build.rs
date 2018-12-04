extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_std_types(false)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("include/point.h");
}
