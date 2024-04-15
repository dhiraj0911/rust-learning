fn main() {
    // let some_number = Some(4);
    // let some_string = Some("this is string lateral");

    // let absent_numbe: Option<i32> = None;


    let x = 3;
    let y = None;

    let sum = x + y.unwrap_or(0);
}   