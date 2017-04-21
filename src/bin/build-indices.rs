#[macro_use] extern crate shells;
extern crate glob;

use glob::glob;

fn main() {
    let files = glob("presentation/toc/*.md").unwrap();
    for file in files {
        let file = file.unwrap();
        let file = file.to_string_lossy();
        let new_name = format!("{}{}", &file.split(".").nth(0).unwrap(), ".html");
        sh!("pandoc -s {} -o {}", file, new_name);
    }
}
