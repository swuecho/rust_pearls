// this is copied from somewhere
//
use std::collections::HashMap;

fn main() {
// type inference lets us omit an explicit type signature (which
// would be `HashMap<&str, &str>` in this example).
let mut book_reviews = HashMap::new();

for _ in 0..1000 {
// review some books.
book_reviews.insert("Adventures of Huckleberry Finn",    "My favorite book.");
book_reviews.insert("Grimms' Fairy Tales",               "Masterpiece.");
book_reviews.insert("Pride and Prejudice",               "Very enjoyable.");
book_reviews.insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.");
}
// check for a specific one.
if !book_reviews.contains_key(&("Les Misérables")) {
    println!("We've got {} reviews, but Les Misérables ain't one.",
             book_reviews.len());
}




}

