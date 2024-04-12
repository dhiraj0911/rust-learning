// Slicing

fn main() {
    let mut s = String::from("Hello World");
    let hello = &s[0..5];   // [..5]
    let world = &s[6..11];  // [6..]
    // s.clear()    Here we cannot do s.clear() because hello and world are immutable reference of s
    println!("{} {}", hello, world);    //ending the scope of hello and world here
    s.clear();
    let word = get_first_word(&s);
    println!("{}", word);
    

    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[0..4];
    
}

fn get_first_word(s: &String) -> usize {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}