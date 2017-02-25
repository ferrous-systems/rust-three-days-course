#!/usr/bin/env rustscript

#[macro_use] extern crate shells;

extern crate glob;
use glob::glob;

use std::fs::rename;

let locales = ["de-DE", "en-US"];

for locale in locales.iter() {
    let files = glob(format!("chapters/{}/*.md", locale).as_ref()).unwrap();
    for file in files {
        let file = file.unwrap();
        let file = file.to_string_lossy();
        let new_name = format!("{}{}", &file.split(".").nth(0).unwrap(), ".chapter");
        sh!("git mv {} {}", file, new_name);
    }
}