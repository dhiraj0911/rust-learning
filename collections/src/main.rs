fn main() {
    let a = [12,43,4,5,45];
    let mut v:Vec<i32> = Vec::new();   
    v.push(12);
    v.push(123);

    let mut v2 = vec![1, 2, 3, 4, 5];

    let third = &v2[2];       // once we have immutable reference of vec we cannot have
                                    // mutable reference of it not even v.push()
    // let third = &v2[23];        //This crashes application at runtime
    // println!("The third number is {}", third);

    // Alternative
    match v2.get(223) {
        Some(third) => println!("The third number is {}", third),
        None => println!("There is no third number")
    }

    for i in &mut v2{
        *i += 50;
    }
    for i in &v2 {
        println!("{}", i);
    }
}