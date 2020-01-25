use std::fs::read_to_string;
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
    let path = Path::new("src/lib/data_03.txt");
    let file_content = read_to_string(path).unwrap();

    // If reading the file to a String worked, it is parsed as &str into serde_json::Value
    let person: Person = serde_json::from_str(&file_content)?;
    Ok(person)
}



fn main() {

    let data = json_validator();
    match data {
        Ok(v) => println!("Please call {} at the number {:?}", v.name, v.phones[0]),
        Err(e) => println!("error: {:?}", e),
    }
}
