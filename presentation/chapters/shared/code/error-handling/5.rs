fn multiple_possible_failures() -> Result<String,String> {
    this_can_fail(true)?;
    println!("Nach dem ersten Fehler");
    this_can_fail(false)?;
    println!("Nach dem zweiten potentiellen Fehler");
    Ok(String::from("Alles erledigt"))
}

fn main() {
    multiple_possible_failures();
}