use std::{fs::File, io::ErrorKind};

fn main() {
    // panic!("this is error");


    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E)
    // }
    
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("this is error while creating file: {:?}", error)
            }
            other_error => {
                panic!("Error in reading file: {:?}", other_error)
            }
        }
    };
}