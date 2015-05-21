fn main() {

let (x, y) = (1,2);
let mut count = 0;

while count < 10 {
    println!("count: {}", count);
    count += 1;
    println!("count: {}", x);
    println!("count: {}", y);
}
}
