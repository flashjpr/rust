pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

// brings into scope only the 'of' module
use a::series::of;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow}; // or bring everything using '*' -- TrafficLight::*;


fn main() {
    // the verbose way is the same as
    a::series::of::nested_modules();

    // -> this
    of::nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
