use std::{fs::File, io::ErrorKind};
use std::io;
use std::io::Read;

fn main() {
    // panic!("this is error");


    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E)
    // }

    let f = File::open("hello.txt");
    
    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("this is error while creating file: {:?}", err)
            }
            other_error => {
                panic!("Error in reading file: {:?}", other_error)
            }
        }   
    };

    // Using `unwrap` function

    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("error in reading file");
}


fn read_username_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("hello.txt")?;

    // // let mut f = match f{
    // //     Ok(file) => file,
    // //     Err(err) => Err(e)
    // // };

    // let mut s = String::new();

    // // match f.read_to_string(&mut s) {
    // //     Ok(_) => Ok(s),
    // //     Err(e) => Err(e)
    // // }
    // f.
    // Ok(s)

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

    // ? operator only works where function return Result of Options
}