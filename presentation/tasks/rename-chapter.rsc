#!/usr/bin/env rustscript

#[macro_use] extern crate shells;
use std::fs::rename;

let locales = ["de-DE", "en-US"];
let original = std::env::args().nth(1).unwrap();
let new = std::env::args().nth(2).unwrap();

for locale in locales.iter() {
    let originalpath = format!("chapters/{}/{}.chapter", locale, original);
    let newpath = format!("chapters/{}/{}.chapter", locale, new);
    rename(&originalpath, &newpath).expect("File renaming must be successful");
    sh!("git rm {}", originalpath);
    println!("Renamed {} to {}", originalpath, newpath);
}