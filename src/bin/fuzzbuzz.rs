// how to go from u32 => &'a static str 
// to avoid the to_string() all over the place?
fn main() {
    for num in 1u32..101u32 {
        let  answer = 
         if  num % 15 == 0 {
             "FizzBuzz".to_string()
         }
         else if num % 3 == 0 {
             "Fizz".to_string()
         }
         else if num % 5 == 0 {
             "Buzz".to_string()
         }
         else {
            num.to_string()
         };

         println!("{}", answer);
    }
}

