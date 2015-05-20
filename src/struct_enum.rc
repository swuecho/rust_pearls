use std::f64::consts::PI;
//use self::Shape; how to make it work?
#[derive(Debug)]
struct Point { x: f64,
               y: f64
            }


enum Shape<'a> {
    Circle(&'a Point, f64), // how to interpret this?
    Rectangle(&'a Point, &'a Point)
}

fn surface(s: &Shape) -> f64 {
    match *s {
        Shape::Circle( _,  r) => { PI * r * r }
        Shape::Rectangle( p1, p2) => { ((p1.x - p2.x) * (p1.y - p2.y)).abs() }
    }
}


fn main() {
    let mp1 = Point { x: 10.1 , y : 10.2 };
    let mp2 = Point { x: 1.1 , y : 12.2 };
    let sh_c = Shape::Circle(&mp1,2.5);
    let sh_r = Shape::Rectangle(&mp1,&mp2);
    println!("the area is {:?}", &mp1);
    println!("the area is {}", surface(&sh_c));
    println!("the area is {}", surface(&sh_r));
}

