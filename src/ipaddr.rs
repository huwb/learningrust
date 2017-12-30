#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind {
    fn ping(&self) -> () {
        match self {
            &IpAddrKind::V4(_, _, _, _) => println!("Old net: {:?}", self),
            &IpAddrKind::V6(_) => println!("New net: {}", 1),
        }
    }
}

fn main() {
    let four = IpAddrKind::V4(4, 6, 8, 9);
    let six = IpAddrKind::V6(String::from("doob"));

    // println!("IP {:?}", six);

    four.ping();
    six.ping();
    // ping(four);
    // ping(six);
}
