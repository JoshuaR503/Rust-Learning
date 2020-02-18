#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {

    // We can create instances of each of the two variants of IpAddrKind like this:
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("\n\nhome: {:#?}\nloopback: {:#?}\n\n", home, loopback);
}
