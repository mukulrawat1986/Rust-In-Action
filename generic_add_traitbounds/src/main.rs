use std::ops::Add;

fn main() {
    let (a, b) = (1.2, 3.4);
    let (x, y) = (10, 20);

    let c = add(a, b);
    let d = add(x, y);

    println!("{} + {} = {}", a, b, c);
    println!("{} + {} = {}", x, y, d);
}

// Our input type T must implement Add, and that implementation outputs a value also of type T
fn add<T: Add<Output = T>>(i:T, j:T) -> T {
    i+j
}