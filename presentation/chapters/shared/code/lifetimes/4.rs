struct Container<'a, T: 'a> {
    inner: &'a T
}

struct Worker<'a, 'b: 'a, T: 'b> {
    workload: &'a Container<'b, T>
}

fn create_worker<'a, 'b: 'a, T>(container: &'a Container<'b, T>) -> Worker<'a, 'b, T> {
    Worker { workload: container }
}

fn main() {
    let item = 42;
    let c = Container { inner: &item };
    let w = create_worker(&c);
}