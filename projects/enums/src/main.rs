enum IpAddrKind {
    V4,
    V6,
}

enum NewIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { z: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let homeEnum = NewIpAddr::V4(127, 0, 0, 1);
    let loopbackEnum = NewIpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}
