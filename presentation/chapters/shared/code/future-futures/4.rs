async fn do_with_str() {
    let x = "foo".to_string();
    let x_ref: &str = &x;
    await!(thing_with_string(x_ref));
}
