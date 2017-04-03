#[macro_use] extern crate shells;
extern crate glob;

use glob::glob;

fn main() {
    let locales = ["de-DE", "en-US", "es-ES"];

    for locale in locales.iter() {
        let files = glob(format!("presentation/chapters/{}/*.md", locale).as_ref()).unwrap();
        for file in files {
            let file = file.unwrap();
            let file = file.to_string_lossy();
            let new_name = format!("{}{}", &file.split(".").nth(0).unwrap(), ".chapter");
            sh!("git mv {} {}", file, new_name);
        }
    }
}