fn main() {
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("we have 2 enum variants returning data of different data types: v4: {:?} and v6: {:?}",home, loopback);
}

//enum defination
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


