async fn get_some_data() -> Result<Data, NetworkError> {
    // ...
}

fn main() -> Result<(), Box<Error>> {
    // ... init run loop

    let data = await!(get_some_data?);
}