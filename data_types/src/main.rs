fn main() {

    // SCALARs
    // ints
    let _guess: isize = "43".parse()
        .expect("NaN");
    
    let _x: isize = <isize>::max_value();
    let _64: i64 = <i64>::max_value();
    
    println!("{}", _x);
    println!("{}", _64);

    // floats
    let x = 2.0; // f64
    println!("{}", x);
    let x: f32 = 3.0;
    println!("{}", x);

    // char : single ''
    let heart_eyed_cat: char = '😻';
    println!("{}", heart_eyed_cat);

    // COMPOUNDs
    // tuples: elements may have different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;  // destructuring the 
    println!("The value of y is: {}", y);

    // destructuring using dot index
    let x: (i32, f64, u8) = (225, 9213.2213, 200);
    println!("{}", x.0);
    println!("{}", x.1);
    println!("{}", x.2);

    // arrays
    let x = [0, 1, 923, 2313, 2831123];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

}
