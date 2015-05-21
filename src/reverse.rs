fn main() {
let reversed: String = "一二三四五六七八九十".chars().rev().collect(); 
// why &'static str can have chars method?
println!("{}",reversed);
}
