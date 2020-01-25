serde_json

1. The function `fn json_validator()` reads a txt-file containing JSON data into a String. The data is parsed into serde_json::Value. The function returns the `Result` type.

Call `fn json_validator` in `fn main()` with a variable. Use the `Result` type with `match`. If the JSON is valid, print the format string: `"Please call {} at the number {}", v[index], v[index]`. You can access the data in the `serde_json::Value` type, by indexing with square brackets containing the names of the appropriate fields. If there is more then one item attached to a field, index with numbers. If the JSON is not valid, print the format string: `"error: {:?}", e`.

2. Try, if the error handling works. Use `data_02.txt`, it contains invalid JSON.

3. `serde_json::Value` only checks, if the &str is correct JSON, but not, if the data has the correct format for our purpose. `data_02.txt` contains invalid JSON.

5.  Create the `struct Person`, which implements the Serialize and Deserialize trait. The `struct` has three fields:  `name: String`, `age: u8` and `phones: Vec<String>`. Parse the the file content as `&str` into the `struct Person`. In the format string, change the indexing appropriate to the data types. What happens, if you add another field in the JSON, but not in the `struct`? Change the age, 43 to "Mary". Change the name of one of the fields!


4. Write a program, that changes the values of the original JSON data. 
