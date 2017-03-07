fn main() {
    let maybe_file = std::fs::File::open("Gibt's nicht!");

    match maybe_file {
        Ok(f) => { println!("Datei geöffnet! Debug: {:?}", f) },
        Err(e) => { println!("Datei nicht geöffnet! Fehler: {:?}", e) }
    }
}