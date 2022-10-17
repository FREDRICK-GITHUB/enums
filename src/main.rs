fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    println!("we have 2 enum instances: v4: {:?} and v6: {:?}",four, six);
}

//enum defination
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// function that has one parameter of type enum IpAddrKind
fn route(ip_type: IpAddrKind) {
    println!("function returns nothing else! Only {:?}", ip_type);
}