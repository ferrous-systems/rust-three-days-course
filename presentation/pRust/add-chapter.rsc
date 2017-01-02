#!/usr/bin/env rustscript

#[macro_use] extern crate shells;
use std::fs::File;

let locales = ["de-DE", "en-US"];
let filename = std::env::args().nth(1).unwrap();

for locale in locales.iter() {
    let path = format!("chapters/{}/{}.md", locale, filename);
    File::create(path).expect("File creation should succeed");
    println!("Created {}", filepath);
}