use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    // create instances
    let home = IpAddrEx::V4(127, 0, 0, 1);
    let loopback = IpAddrEx::V6(String::from("::1"));

    // using the std library
    let v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    assert_eq!("127.0.0.1".parse(), Ok(v4));
    assert_eq!("::1".parse(), Ok(v6));

    assert_eq!(v4.is_ipv4(), true);
    assert_eq!(v6.is_ipv6(), true);

    // Message example
    let m = Message::Write(String::from("whatever"));
    m.call(); // print "ginger"

    // rust does NOT have nulls, but has an enum of None or Some
    let some_number = Some(5);
    let some_string = Some("anything");
    let absent_number: Option<i32> = None; // need to tell the compiler what type of Option<T> we have

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();

    println!("Bitcoin is worth ${}.", value_in_dollars(Coin::Bitcoin));
    println!("Ethereum is worth ${}.", value_in_dollars(Coin::Ethereum));
    println!("Monero is worth ${}.", value_in_dollars(Coin::Monero));
    println!("Litecoin is worth ${}.", value_in_dollars(Coin::Litecoin));

    println!(
        "Value in cents for {}",
        value_in_cents(MetalCoin::Quarter(UsState::Alaska))
    );

    // Matching with Optional<T>
    let five = Some(5);
    let six = plus_one(five);
    println!("5 + 1 = {}", six.unwrap());
    let none = plus_one(None);

    // the '_' placeholder
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => (), // '_' is wildcard, '()' is the unit value
    }

    let some_u8_value = Some(0u8);
    match some_number {
        Some(3) => println!("Yey!"),
        _ => (),
    }

    // same as above - less verbose way (but does not have the same exhaustive search that match enforces)
    if let Some(3) = some_u8_value {
        println!("Yey!");
    } else {
        () // same case as to the match-wildcard
    }

}

enum IpAddrEx {
    // attach data directly to each variant (no need for any extra structs)
    V4(u8, u8, u8, u8),
    V6(String),
}

// here, each variant stores a different amounts and types of values
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // annonymous strcut
    Write(String),              // String
    ChangeColor(i32, i32, i32), // i32 values
}

// the above is similar to defining the structs:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // whatever
        println!("ginger");
    }
}

enum Coin {
    Bitcoin,
    Litecoin,
    Ethereum,
    Monero,
}

fn value_in_dollars(coin: Coin) -> u32 {
    match coin {
        Coin::Bitcoin => 8822,
        Coin::Ethereum => 605,
        Coin::Litecoin => 152,
        Coin::Monero => {
            println!("Lucky day"); // instructions
            263 // still returns an u32
        }
    }
}

#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum MetalCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: MetalCoin) -> u32 {
    match coin {
        MetalCoin::Penny => 1,
        MetalCoin::Nickel => 5,
        MetalCoin::Dime => 10,
        MetalCoin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i8>) -> Option<i8> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
