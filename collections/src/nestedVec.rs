fn main() {
    enum SpreadSheet {
        Int(i32),
        Float(f64),
        String(String),
    }

    let row = vec![
        SpreadSheet::Int(1),
        SpreadSheet::Float(23.2),
        SpreadSheet::String(String::from("good"))
    ];

    // for item in &row {
        match &row[2] {
            SpreadSheet::Int(i) => println!("This is integer {}", i),
            SpreadSheet::Float(f) => println!("This is float {}", f),
            SpreadSheet::String(s) => println!("This is string {}", s),
        }
    // }
}
