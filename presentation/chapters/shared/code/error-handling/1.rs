fn this_can_fail(succeeds: bool) -> Result<String, String> {
    if succeeds {
        Ok(String::from("Es funktionierte!"))
    } else {
        Err(String::from("Es hat nicht funktioniert!"))
    }
}

fn main() {
    let outcome = this_can_fail(true);
}