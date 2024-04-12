fn main() {
    // let mut x = 5;
    // println!("The value of x is {}", x);
    // x = 12;
    // println!("The value of x is {}", x);

    // const COUNTS_LIKES: u32 = 100_000;
    // println!("This is from constant {}", COUNTS_LIKES);
    // // We cannot mutate constants
    // // In const we neet of anotate it's type
    // // We cannot set return value of function as const


    // Shadowing in Rust
    let x = 5;
    println!("This is from first variable {}", x);
    let x = "thisisshowding";
    println!("This is showding of {}", x);


    /*
    Datatypes

    1. Scalar Datatypes => represents single value
    2. Compound datatypes => group of values


    Scalar datatypes
        
    */
    // Integer     => 
    //     defatut integer 32bit (i32) other are i8, i32, i64, i128 signed integer
                                                // u8, u32, u64, u128 unsigned integers

    let a = 93_23;  //Decimal
    let b = 0xff;   //Hex
    let c = 0o23;   //Octal
    let d = 0b1111_000; // binary
    let e = b'A';   //byte

    //Integer overflow
    let f: u8 = 255;        //This range is [0, 256) if it assigned 256 it 0 257 to 1 so on 
    println!("{}", f);


    // Floating-point number
    let g = 8 + 2;
    let h = 9.0 + 2.3;

    let sum = 3.2 + 9.3;
    let sub = 23.3 - 1.4;
    let product = 13 * 23;
    let division = 23 / 43;
    let reminder = 23 % 23;

    // Booleans
    let t = true;
    let u = false;
    
    // Characters
    let hi = 's';
    let hii = "s";
    
    /*
        Componds types
    */

    let tupple = ("This is can handle mumtiply values", 100);
    //Accessing values from tuple
    let (first_string, second_interger) = tupple;
    let first_string1 = tupple.0;
    let second_interger1 = tupple.1;

    
    //Array 1
    let array = [12, 423, 432];
    let first_element = array[0];

    //Array 2

    let byte = [3; 8];  //8 size array all set to 3

    println!("{}",byte[4]);
    
    let str = "this:";

    my_function(12, "this is string", 32);
    let sum = my_function1(12, "this is string", 32);

    println!("The sum is {}", sum);
    control_flow();
    loops();
    while_loop();
    for_loop();
}


//Functions

fn my_function(x: i32, str: &str, y: i32) -> i32{
    println!("This is from function");
    println!("{}", x);
    println!("{}", str);
    let sum = x + y;
    return sum;
}
//Other way to retun
fn my_function1(x: i32, str: &str, y: i32) -> i32{
    println!("This is from function");
    println!("{}", x);
    println!("{}", str);
    x + y
}


fn control_flow() {
    let number = 23;
    if number > 0 {
        println!("Positve number");
    }
    else if number < 0 {
        println!("Negative number");
    }
    else {
        println!("The number is zero");
    }

    let condition = true;
    let number = if condition {100} else {0};
    println!("{}", number);
}


fn loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("{}", result); 
}

fn while_loop() {
    let mut number = 0;
    while number <= 10 {
        println!{"{}", number};
        number += 1;
    }
}

fn for_loop() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    for element in arr.iter() {
        println!("{} from for loop", element);
    }

    for number in 1..5 {
        println!("{} from range", number);
    }
}