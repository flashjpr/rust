fn main() {

    // define a struct (similar to creating an object in js)
    struct User {
        name: String,
        email: String, 
        active: bool,
        sign_in_count: u64,
    } // expression, no ';'

    // instantiate a struct 
    let user = User {
        name: String::from("Name 1"),
        email: String::from("name@domain.com"),
        active: true,
        sign_in_count: 2311,
    }; // statement, ';'

    let mut user2 = User {
        name: String::from("Name 2"),
        email: String::from("name2@domain.com"),
        active: false,
        sign_in_count: 0,
    }; 

    user2.active = true;

    println!("User details: {} {} {} {}", user.name, user.email, user.active, user.sign_in_count);
    println!("User details: {} {} {} {}", user2.name, user2.email, user2.active, user2.sign_in_count);


    // builder 
    fn build_user(name: String, email: String) -> User {
        User {
            name,           // or   name: name,
            email,          // or   email: email,
            active: false,
            sign_in_count: 0,
        }
    }

    let user3: User = build_user(String::from("Functional User Name"), String::from("fusrname@domain.com"));
    println!("User details: {} {} {} {}", user3.name, user3.email, user3.active, user3.sign_in_count);


    // struct update syntax
    let user4 = User {
        name: String::from("John"),
        email: String::from("john@domain.com"),
        active: user2.active,
        sign_in_count: user.sign_in_count
    };
    println!("User details: {} {} {} {}", user4.name, user4.email, user4.active, user4.sign_in_count);

    // struct update syntax on steroids
    let user5 = User {
        name: String::from("Doe"),
        email: String::from("doe@domain.com"),
        ..user2 // take the rest of information from user2 ; lol; this is good;
    };
    println!("User details: {} {} {} {}", user5.name, user5.email, user5.active, user5.sign_in_count);



    // tuple structs without named fields
    struct Point(i32, i32, i32);
    struct Color(i32, i32, i32);

    let origin = Point(0, 0, 0);
    let black = Color(0, 0, 0);

    
}
