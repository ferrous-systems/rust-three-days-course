use std::fs::{self, File, read_to_string};
use std::io::prelude::*;
use std::path::Path;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn json_validator() -> Result<Person> {
    // Read a txt-file that contains JSON data into a String.
    let path = Path::new("src/lib/data_01.txt");
    let file_content = read_to_string(path).unwrap();

    // If reading the file to a String worked, it is parsed as &str into serde_json::Value
    let person: Person = serde_json::from_str(&file_content)?;

    Ok(person)
}



fn main() {

    let mut person = json_validator().unwrap();


    person.name = "John Snow".to_string();
    person.age = 44_u8;
    person.phones[1] = "+44 445678".to_string();

    // Serialize it to a JSON string, print to file
    let person_json = serde_json::to_string(&person).unwrap();
    fs::write("person.txt", person_json).unwrap();



}
