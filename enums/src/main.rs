
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin{
    fn dollars(&self)->f32{
        match self {
            Penny => 0.01,
            _ => 0.0,
        }
    }
}

fn cents_in_coin(coin: Coin)->i8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Quarter(state) => { 
            println!("State: {:?}!", state);
            25 },
        _ => 50,
    }
}


fn main() {
    let penny = Coin::Penny;
    println!("Cents in penny: {}", cents_in_coin(penny));

    let nickel = Coin::Nickel;
    println!("Cents in nickel: {}", cents_in_coin(nickel));

    let quarter_from_alabama = Coin::Quarter(UsState::Alabama);
    println!("Cents in quarter_from_alabama: {}", cents_in_coin(quarter_from_alabama));

    let quarter_from_alaska = Coin::Quarter(UsState::Alaska);
    println!("Cents in quarter_from_alaska: {}", cents_in_coin(quarter_from_alaska));


}
