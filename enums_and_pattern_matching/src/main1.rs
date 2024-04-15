enum IpAddKind {
    // V4(String),             //To add data inside enum we need () 
    V4(i8, i8, i8, i8),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},      //IT store anonymour struct
    Write(String),
    ChangeColor(i32, i32, i32)
}

// We can define it in individual struct but it will have different type
// What enum gives us that all varience are grouped under this type

impl Message {
    fn some_function() {
        println!("hii");
    }
}

struct IpAddr {
    kind: IpAddKind,
    address: String,

}

fn main() {
    let four = IpAddKind::V4; //Varience are under namebased under identifier that's why we use ::
    let six = IpAddKind::V6;


    // let localhost = IpAddr {
    //     kind: IpAddKind::V4,
    //     address: String::from("127.0.0.1")
    // }

    // let localhost = IpAddKind::V4(String::from("127.0.0.1"));
    let localhost = IpAddKind::V4(127, 0, 0, 1);


}

fn route(ip_kind: IpAddKind) {}