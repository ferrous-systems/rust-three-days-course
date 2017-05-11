fn foo(n: u32) -> impl Iterator<Item=u32> {
    (0..n).map(|x| x * 100)
}

for x in foo(10) {
    // x = 0, 100, 200, ...
}