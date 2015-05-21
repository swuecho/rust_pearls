// this maybe copied from somewhere, but is a very good 
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
// fn area(self) -> f64 { 
// the above line works too

// Also, if the type of the expression to the left of the dot is a pointer,
// it is automatically dereferenced to make the field access possible.
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    let c = Circle { x: 0.5, y: 0.8, radius: 2.0 };
    let d = &c;  // make an alias, acutally, take the address:w
    println!("{}", c.x);
    println!("{}", d.y); // autoderef
    println!("{}", (*d).y);
    //println!("{}", *d.y); this is wrong, because . have higher precdence
    println!("{}", c.area());
}
