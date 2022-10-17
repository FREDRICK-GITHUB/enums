fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("we have 2 struct instances: v4: {:?} and v6: {:?}",home, loopback);
}

//enum defination
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}


