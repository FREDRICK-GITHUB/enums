fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("The value of five is: {:#?}", &five);
    println!("The value of six is: {:?}", &six);
    println!("The value of none is: {:?}", &none);
}


fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}



