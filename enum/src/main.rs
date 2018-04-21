use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    // create instances
    let home = IpAddrEx::V4(127,0,0,1);
    let loopback = IpAddrEx::V6(String::from("::1"));

    // using the std library
    let v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    assert_eq!("127.0.0.1".parse(), Ok(v4));
    assert_eq!("::1".parse(), Ok(v6));

    assert_eq!(v4.is_ipv4(), true);
    assert_eq!(v6.is_ipv6(), true);
}

enum IpAddrEx {
    // attach data directly to each variant (no need for any extra structs)
    V4(u8,u8,u8,u8),
    V6(String),
}




