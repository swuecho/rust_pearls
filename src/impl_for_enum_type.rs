// implement custom display for enum type
// reference: LYHG p140
use std::fmt;

enum TrafficLight { Red, Yellow, Green }

impl fmt::Display for TrafficLight {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match *self {
            TrafficLight::Red    =>  "Red Light",
            TrafficLight::Yellow =>  "Yellow Light",
            TrafficLight::Green  =>  "Green Light",
        };
        write!(f, "{}",display)
     }

}

fn main() {
    let a = TrafficLight::Red;
    let b = TrafficLight::Yellow;
    let c = TrafficLight::Green;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}
