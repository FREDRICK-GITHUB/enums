fn main() {
    
    // first struct instance
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    //second struct instance
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };


    println!("we have 2 struct instances: v4: {:#?} and v6: {:#?}",home, loopback);
}

//enum defination
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// struct defination that implements above enum
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
