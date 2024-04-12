fn main() {

    //Find length of String
    // let s1 = String::from("hello");
    // let (s2, len) = get_length_without_own(s1);
    // println!("{} {}", s2, len);

    //let's fix this

    // References don't take ownership of the underlying values
    // references are immutable by default  (We can't modify it)
    // Better version
    let s1 = String::from("hello");
    let len = get_length_without_own(&s1);
    println!("{} {}", s1, len);


    //now suppose we wanted to modify string without taking ownerhsip of it
    // by passing by reference we can't able to modity so solution is by making strgin mut

    let mut new_str = String::from("hi");
    change_fun(&mut new_str);
    println!("{}", new_str);

    // let new_ref = &mut new_str;
    // let new_ref1 = &mut new_str;    // This will not possible because on can have only once mutable reference in the scope
    // println!("{} {}", new_ref, new_ref1);
    
    //For one variable we can make only one mutable referece at a scope
    // Biggest advantages for this it will avoid race condition
    

    /*
    Rules of references

    1. At any give time, you can either have one mutable reference
    or multiple immutable references.
    
    2. References must be always valid.

    */

    let mut s = String::from("this is okay");

    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s;    //Here we got the error because 
    // immutable reference already exist before mutable reference
    println!("{} {}", s1, s2);
    let s3 = &mut s;    //Here you can have one immutable reference of s because s1 and s2 are out of scope now
    println!("{}",  s3);
    //we can add third mutable reference undernith
    // here because s1 and s2 and s3 are out of scope
    let s4 = &mut s;
    s4.push_str("hii");
    println!("{}", s);


    //Dangling References
    // what happens if our pointer points to invalid data

    // let reference_to_nothing = dangle();
    // After executing dangle() function it return a reference to string but
    // Hovever s is defined within that scope it means that it will delete that string
    // from heap therefore it will point to invalid but Rust will warn us

}


// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// fn get_length_without_own(s: String) -> (usize, String) {
//     let length = s.len();
//     (len, s)
// }

fn get_length_without_own(s: &String) -> usize {
    let length = s.len();
    length
}

fn change_fun(s: &mut String) {
    s.push_str(" world");
}

