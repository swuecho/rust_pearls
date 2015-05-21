#[derive(Debug)]
struct Person {
    first_name : &'static str,
    last_name  : &'static str,
    age        : u8,
    height     : f32,
    phone_number : &'static str,
    flavor       : &'static str
}


fn main() {
    let p = Person { first_name: "Hao"
                    ,last_name : "Wu"
                    ,age       :  25
                    , height   : 20.1
                    , phone_number : "3344440187"
                    , flavor    : "great"
    };
    println!("{:?}", p);
    println!("{}", p.first_name);
}
