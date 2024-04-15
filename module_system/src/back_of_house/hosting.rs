pub struct Breakfast {
    pub toast: String,
    fruit: String
}

pub enum Types {
    Veg,    //For the this is by default public 
    NonVeg  // don't need pub keyworda
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            fruit: String::from("Papaya")
        }
    }
}