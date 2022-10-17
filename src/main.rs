fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("we have 2 enum instances: v4: {:?} and v6: {:?}",four, six);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}