
fn main() {
    //The if let example here is used when we only want to cater for one pattern and do nothing for the other patterns
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

}





