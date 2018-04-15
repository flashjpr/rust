fn main() {

    //  vars
    let mut x = 5;
    println!("Value of x is : {}", x);

    x = 6;
    println!("Value of x is : {}", x);

    // consts
    const MAX_POINTS: u32 = 100_000;
    println!("Value of MAX_POINTS is {}", MAX_POINTS);

    // uuuh, shadows! 
    let s = 1;
    let s = s + 2;
    let s = s + 3;
    println!("Value of s is {}", s);

    // bcz it's a new var the type can be changed
    let spaces = "       ";
    let spaces = spaces.len();
    println!("Number of spaces is {}", spaces);

    // for a mut var the type cannot be change 
    let spaces = "    ";
    // spaces = spaces.len(); -> this would error due to mismatched types

    
}
