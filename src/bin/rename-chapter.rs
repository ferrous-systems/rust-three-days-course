#[macro_use] extern crate shells;
extern crate tasks;

use tasks::LOCALES;
use std::fs::rename;

fn main() {
    let original = std::env::args().nth(1).expect("Expected an original name for the chapter.");
    let new = std::env::args().nth(2).expect("Expected a new name for the chapter.");

    for locale in LOCALES.iter() {
        let original_path = format!("presentation/chapters/{}/{}.chapter", locale, original);
        let new_path = format!("presentation/chapters/{}/{}.chapter", locale, new);
        rename(&original_path, &new_path).expect("File renaming must be successful");
        sh!("git rm {}", original_path);
        println!("Renamed {} to {}", original_path, new_path);
    }
}
