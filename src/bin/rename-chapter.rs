#[macro_use] extern crate shells;

use std::fs::rename;

fn main() {
    let locales = ["de-DE", "en-US", "es-ES"];
    let original = std::env::args().nth(1).expect("Expected an original name for the chapter.");
    let new = std::env::args().nth(2).expect("Expected a new name for the chapter.");

    for locale in locales.iter() {
        let originalpath = format!("presentation/chapters/{}/{}.chapter", locale, original);
        let newpath = format!("presentation/chapters/{}/{}.chapter", locale, new);
        rename(&originalpath, &newpath).expect("File renaming must be successful");
        sh!("git rm {}", originalpath);
        println!("Renamed {} to {}", originalpath, newpath);
    }
}
