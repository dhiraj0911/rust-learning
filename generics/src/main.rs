fn main() {
    let number_list = vec![1, 43, 2, 4, 53, 34];
    println!(
        "Max number from 1st one is {}",
        find_max_number(number_list)
    );

    let number_list1 = vec![123, 43, 23, 1232];
    println!("THis is from 2nd {}", find_max(number_list1));

    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("Largest char is {}", find_max(char_list));

    // I cannot use same function for finding max char using same function
    // this is where `generic` comes into picture
    // let's convert following function to generic type
    test();
}

fn find_max_number(number_list: Vec<i32>) -> i32 {
    let mut max_num = number_list[0];
    for number in number_list {
        if number > max_num {
            max_num = number;
        }
    }
    max_num
}

//This means traits
fn find_max<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut max_num = number_list[0];
    for number in number_list {
        if number > max_num {
            max_num = number;
        }
    }
    max_num
}

// struct Point<T, U> {
//     x: T,
//     y: U
// }

// fn two_generic_types() {
//     let p1 = Point {x: 3, y: 3};
//     let p1 = Point {x: 3.3, y: 3.2};
//     let p1 = Point {x: 3, y: 3.3};
// }

enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}

// Write a method on struct using impl

struct Point<T> {
    x: T,
    y: T,
}

impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

struct Point1<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point1<T, U> {
    fn mix_up<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

fn test() {
    let x = Point1 { x: 2, y: 4 }; //IT has access of only x()
    let x1 = Point1 { x: 2.4, y: 4.23 }; //IT has acess of both x() and y()

    //Complex

    let x3 = x.mix_up(x1);
    println!("x: {}, y: {}", x3.x, x3.y);
}
