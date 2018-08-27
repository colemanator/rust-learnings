use std::io;

fn main() {
    loop {
        println!("Select a coin to get it's value:");

        println!("1 => Penny");
        println!("2 => Nickel");
        println!("3 => Dime");
        println!("4 => Quarter");

        println!("Enter the corresponding number: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let option = input.chars().next().unwrap();

        let coin = match option {
            '1' => Coin::Penny,
            '2' => Coin::Nickel,
            '3' => Coin::Dime,
            '4' => Coin::Quarter,
            _   => break
        };

        println!("The coin selected is worth: {}\n\n", value_in_cents(coin));
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    } 
}
