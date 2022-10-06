enum IpAddressKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddressKind,
    address: String,
}

// We can pass string directly to enum
enum IpAddrNew {
    V4(String),
    V6(String),
}

fn enum_example() {
    let four = IpAddressKind::V4; // New instance ipv4
    let six = IpAddressKind::V6; // New instance ipv6

    // can pass both instances because of same type IpAddressKind
    route(IpAddressKind::V4);
    route(IpAddressKind::V6);

    let home = IpAddr {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };

    let home_new = IpAddrNew::V4(String::from("127.0.0.1"));
    let loopback_new = IpAddrNew::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddressKind) {}

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// Rust do not have null
// enum Option<T> {
//     None,
//     Some(T),
// }
