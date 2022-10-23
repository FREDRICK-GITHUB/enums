/*
The _ placeholder is used as a pattern to cater for all the other scenarios that are not defined in the preceeding patterns */
fn main() {
//    let some_u8_value = 7u8;
//    match some_u8_value {
//     1 => println!("one"),
//     3 => println!("three"),
//     5 => println!("five"),
//     7 => println!("seven"),
//     _ => println!(),
//    }

   let some_u8_value = Some(3u8);
   match some_u8_value {
    Some(1) => println!("one"),
    Some(3) => println!("three"),
    Some(5) => println!("five"),
    Some(7) => println!("seven"),
    _ => println!(),
   }

}





