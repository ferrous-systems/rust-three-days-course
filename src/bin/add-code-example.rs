extern crate glob;
extern crate tasks;

use tasks::LOCALES;

use glob::glob;
use std::fs::{File, rename};
use std::path::PathBuf;
use std::io::{Read, Write};

fn example_path_to_numeric(val: &PathBuf) -> usize {
    val.file_stem().expect("Could not get stem of file.")
        .to_str().expect("Could not convert OsStr to str")
        .parse::<usize>().expect("Could not parse str as number")
}

// This task inserts a code example into a chapter at the given index.
// For example:
//      cargo run --bin add-code-sample borrowing 2
// Would shift the example presentation/chapters/shared/code/{2,3,...}.* up by one (to 3,4,...)
// as well as alter references in the slides.
fn main() {
    // Args:
    //  0: Executable
    let mut args = std::env::args().skip(1);
    //  1: Chapter
    let chapter = args.next()
        .expect("No chapter provided.");
    //  2: Code example to start shifting from.
    let example = args.next()
        .expect("No example index provided.")
        .parse::<usize>()
        .expect("Index provided was not an integer.");

    // This globs any file in the range from the specified index up in the chapter.
    let mut entries = glob(&format!("presentation/chapters/shared/code/{}/*", chapter))
        .expect("Could not read chapter folder.")
        .map(|v| v.unwrap()) // We want to see the error.
        .collect::<Vec<_>>();
    
    // Sort the entries numerically.
    entries.sort_by(|a,b| {
        let (a_stem, b_stem) = (example_path_to_numeric(a), example_path_to_numeric(b));
        a_stem.cmp(&b_stem)
    });

    // Rename them. Starting highest to lowest.
    let mappings = entries.into_iter().skip(example).map(|entry| {

        // First deal with the file name. Need to get the stem so we can parse and increment it.
        let file_name = String::from(entry.file_name()
            .unwrap()
            .to_str()
            .unwrap());
        let split_name = file_name.split('.').collect::<Vec<_>>();
        let stem = split_name.get(0)
            .expect("Example did not have a stem and was assumed to.");
        let extension = split_name.get(1)
            .expect("Example did not have extension and was assumed to.");

        // This is the new stem for the file.
        let new_index = stem.parse::<usize>()
            .expect("Example was not named with an integer and was assumed to.") + 1;

        // Name a new file name.
        let mut new_entry = entry.clone();
        new_entry.set_file_name(format!("{}.{}", new_index, extension));

        (String::from(entry.to_str().unwrap()), String::from(new_entry.to_str().unwrap()))
    }).collect::<Vec<_>>();

    for &(ref entry, ref new_entry) in mappings.iter().rev() {
        // Move it.
        println!("Renaming {:?} to {:?}", entry, new_entry);
        rename(entry, new_entry).expect("Could not rename.");
    }

    // Now we need to go and mangle the slides.
    for locale in LOCALES.iter() {
        let file_location = format!("presentation/chapters/{}/{}.chapter", locale, chapter);
        let mut file = match File::open(file_location.clone()) {
            Ok(f) => f,
            Err(e) => {
                println!("Couldn't open slides of locale {} for reading: {:?}", locale, e);
                continue
            },
        };
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Couldn't read slides");

        for &(ref entry, ref new_entry) in mappings.iter().rev() {
            let trimmed_entry = entry.replace("presentation/", "");
            let trimmed_new_entry = new_entry.replace("presentation/", "");

            contents = contents.replace(&trimmed_entry, &trimmed_new_entry);
            println!("Replaced {} with {} in {} slides.", trimmed_entry, trimmed_new_entry, locale);
        }

        let mut file = File::create(file_location)
            .expect("Couldn't open slides for writing.");
        file.write_all(contents.as_bytes())
            .expect("Could not write modified slides");
    }
}