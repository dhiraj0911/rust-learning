use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut mp = HashMap::new();
    mp.insert(blue, 40);
    mp.insert(yellow, 60);       //here are transfering ownership of strings

    let color_name = String::from("Blue");
    let intensity = mp.get(&color_name);

    // match intensity {
    //     Some
    // }

    for (key, value) in &mp {
        println!("{} {}", key, value);
    }


    mp.insert(String::from("blue"), 10);
    mp.insert(String::from("blue"), 39);    //THis will override the value

    mp.entry(String::from("yellow")).or_insert(30); // If yellow exists do nothing other wise make enty of yellow with 30
    mp.entry(String::from("yellow")).or_insert(40);     // THis will do nothing


    let text = "hello world good world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}