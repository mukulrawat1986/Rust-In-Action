fn main() {
    // types can be inferred by the compiler
    let a = 10;

    // types can be declared by the programmer when creating
    // variables
    let b: i32 = 20;

    let c = add(a, b);
    println!("a + b = {}", c)
}

// types are required when defining functions
fn add(i: i32, j: i32) -> i32 {
    i + j // function returns the last expressions result, so return
          // is not required.
}