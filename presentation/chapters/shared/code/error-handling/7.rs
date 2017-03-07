fn main() {
    let some_result = this_can_fail(true);
    // Dies wird nur ausgeführt, wenn `some_result` eine `Ok`-Variante ist.
    let mapped_result = some_result.map(|val| val.len());
    println!("{:?}", mapped_result);
}