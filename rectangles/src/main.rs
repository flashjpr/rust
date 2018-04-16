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

    // 3th try - calculating the area using a self Rect struct method
    let rect3 = Rectangle {
        width: 32,
        height: 82,
    };

    println!(
        "The are of the rectangle is {} square pixels.",
        rect3.area()
    );

    // can a rect fit other rect?
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(32); // associated fcts are called using the :: syntax
    println!("The area of rect3 (square of 32) is {}", rect4.area());
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

// define a method "on" the Rectangle struct
impl Rectangle {
    // implementation block
    fn area(&self) -> u32 {
        // borrowed as taking ownership is not intended
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

// associated functions (don't take self as a param)
impl Rectangle { // multiple 'impl' blocks
    fn square(size: u32) -> Rectangle {
        Rectangle {
            // create a square
            width: size,
            height: size,
        }
    }
}
