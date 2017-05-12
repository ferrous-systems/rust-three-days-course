extern crate tasks;

use tasks::LOCALES;

use std::fs::File;

fn main() {
    let filename = std::env::args().nth(1)
        .expect("No chapter name passed! Aborting...");

    for locale in LOCALES.iter() {
        let path = format!("presentation/chapters/{}/{}.chapter", locale, filename);
        File::create(&path).expect("File creation should succeed");
        println!("Created {}", &path);
    }
}
