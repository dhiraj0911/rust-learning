// Combination of match expression and enum

fn main() {
    let five = plus_one(Some(4));

    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?}", five, six);


    let some_value = Some(4);
    match some_value {
        Some(4) => println!("four!!"),
        _ => ()
    }

    //If alternative

    if let Some(4) = some_value {
        println!("four")
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None => None,
        Some(i) => Some(i + 1),
        _ => None       // It doesn't matches with anyother then execute this
    }
}