fn main() {
    value_in_cents(Coin:: Quater(UsState::Alaska));
}

#[derive(Debug)]
enum UsState {
    Carlifornia,
    NewYork,
    Arizona,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin:: Penny => 1,
        Coin:: Nickel => 4,
        Coin:: Dime => 16,
        Coin:: Quater(state) => {
            println!("State quater from {:?}!", state);
            23
        }
    }
}