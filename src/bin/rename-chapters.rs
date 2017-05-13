#[macro_use] extern crate shells;
extern crate glob;
extern crate tasks;

use tasks::LOCALES;

use glob::glob;

fn main() {
    for locale in LOCALES.iter() {
        let files = glob(format!("presentation/chapters/{}/*.md", locale).as_ref()).unwrap();
        for file in files {
            let file = file.unwrap();
            let file = file.to_string_lossy();
            let new_name = format!("{}{}", &file.split(".").nth(0).unwrap(), ".chapter");
            sh!("git mv {} {}", file, new_name);
        }
    }
}