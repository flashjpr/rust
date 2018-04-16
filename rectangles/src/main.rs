fn main() {
    // 1st try - tuples
    let width = 30;
    let height = 50;
    println!(
        "The are of the rectangle is {} square pixels.",
        area(width, height)
    );

    // 2nd try - tuples with more meaning
    let rect1 = (30, 50); // tuple of 30, 50
    println!(
        "The are of the rectangle is {} square pixels.",
        area2(rect1)
    );

    // 3rd try - more organized strcts
    let rect2 = Rectangle {
        width: 32,
        height: 82,
    };

    println!(
        "The are of the rectangle is {} square pixels.",
        area3(&rect2) // only read only, so pass a reference to it
    );

    println!("Rect 2 is {:#?}", rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// borrow the struct rather than take ownership of it!
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
