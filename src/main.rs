fn main() {
    println!("The highest cent denomination is Quarter and has a value of: {}", value_in_cents(Coin::Quarter));
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}


fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        /*first part of argument before => is called a pattern. Second part is the code to run
        we have 4 arms in this enum (Coin)
        */
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => { 
            println!("kanuthu!!!");
            25 
        },
    }
}

