fn main() {
    // create instances
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

enum IpAddr {
    V4(String),
    V6(String),
}




