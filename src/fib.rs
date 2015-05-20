fn main() {
 
   fn fib(n: u64) -> u64 {
 
     fn _fib(n: u64, a: u64, b: u64) -> u64 {
       match (n, a, b) {
             (0, _, _) => a,
              _        => _fib(n-1, a+b, a)
       }
     }
 
     _fib(n, 0, 1)
 
   }
 
   for n in 0u64..50u64  {
     println!("{}", fib(n))
   }
}
