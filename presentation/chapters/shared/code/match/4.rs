fn main() {
    let maybe_file = std::fs::File::open("Gibt's nicht!");

    match maybe_file {
        Err(e) => { println!("Datei nicht geöffnet! Fehler: {:?}", e) }
        _ => {}
    }
}