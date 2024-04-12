fn main() {
    // Ownership is what allow rust to make memory safty
    // garantees without the use of garbage collector


    //Garbage collector used by java
    //Manual memory management used by c/c++
    //Ownership model   => rust data

/*
    Ownership model is way to manage memory
*/

    //  ----Ownership Rule-----
    // 1. Each value in Rust has a variable that's called it's owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.


    {   // s is not valid here
        // let s = "this is only accesible in this scope";  // s is valid from this point
        // let s1 = String::from("hello"); // dynamic in size
        // do stuff with s
    }   // the scope is over, s will no longer valid

    // here s is string literal and which is directly store in binary in stack
    // on the other hand s1 is string type which is dynamic in size
    // and it will be store in heap;


    // in c++ for dynamic memory we need new and delete key word to allocate and deallocate respectively.
    // but in rust will automatically allocate memory and deallocate once it is out of scope.




    let s1 = String::from("hello");
    // let s2 = s1;    // This is not shallow copying (MOVE)
    // let _s2 = s1.clone();
    // println!("{}", s2);


    let x = 5;
    let y = x;  // It will be copy here not Move

    let str1 = "hello";
    let str2 = str1;    //Not Move (Shallow copying)
    println!("{}", str1);


    let line = String::from("Dhiraj");
    takes_ownership(line);
    // println!(line);

    makes_copy(x);
    println!("{}",x);


    let got_ownership = give_ownership();
    println!("{}", got_ownership);


    //takes and give back
    let s3 = String::from("this is dhiraj");
    let s4 = takes_and_gives(s3);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn give_ownership() -> String {
    String::from("this is from give_ownership fn")
}

fn takes_and_gives(a_string: String) -> String{
    a_string
}