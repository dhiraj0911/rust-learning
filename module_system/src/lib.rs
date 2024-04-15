mod front_of_house {
    // By default all the child module in side an module
    // are private we need to provide pub keyword before that
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //Aboslute path to access module function
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn main() {
    
}


// Using superkey word
fn server_order() {}

mod back_to_order {
    fn fix_order() {
        cook_order();
        super::server_order();
        // `super` key word allow us to reference patent moduel
    }

    fn cook_order() {}
}