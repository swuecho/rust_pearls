use std::env;
fn main() {
    println!("Program: {}", env::args().nth(0).unwrap());
}
