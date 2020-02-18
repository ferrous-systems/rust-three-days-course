use std::io::{Read, BufReader, BufRead, Lines};
use std::fs::File;

fn is_line_empty(line: String) -> Option<String> {
    if line.len() == 0 {
        None
    } else {
        Some(line)
    }
}

fn main() {

    let f = File::open("src/lib/content.txt");

    let file = match f {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };


    let mut buf_reader = BufReader::new(file).lines();

    for line in buf_reader {

        let line = match line {
            Ok(content) => content,
            Err(e) => panic!("Problem opening the file: {:?}", e),
        };
        
        let line = is_line_empty(line);

        match line {
            Some(line) => println!("{}", line),
            None => continue
        };
    }
    println!("{}", number);
}
