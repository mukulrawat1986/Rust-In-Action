fn main() {
    let twenty = 20;
    let twenty_one: i32 = twenty + 1;
    let floats_okay = 21.0;

    // underscores are used to increase visibility and are ignored by the compiler
    let one_million = 1_000_000;

    println!("{}; {}; {}; {}", twenty, twenty_one, floats_okay, one_million);
}