use std::fs::read_to_string;
use std::path::Path;
use serde_json::{Result, Value};


fn json_validator() -> Result<Value> {
    // Read a txt-file that contains JSON data into a String.
    let path = Path::new("src/data.txt");
    let file_content = read_to_string(path).unwrap();

    // If reading the file to a String worked, it is parsed as &str into serde_json::Value
    let json_data: Value = serde_json::from_str(&file_content)?;
    Ok(json_data)
}

fn main() {

    let data = json_validator();
    match data {
        Ok(v) => println!("Please call {} at the number {}", v["name"], v["phones"][0]),
        Err(e) => println!("error: {:?}", e),
    }
}
