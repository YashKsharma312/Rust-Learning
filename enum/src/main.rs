#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
#[derive(Debug)]
enum IpAddrr {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
 

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}",loopback.kind);

    let home1 = IpAddrr::V4(127, 0, 0, 1);

    let loopback1 = IpAddrr::V6(String::from("::1"));
    println!("{:?}",loopback1);
    println!("{:?}",home1);
}
