mod back_of_house;

//Use bring items into scope of our program
pub use back_of_house::hosting::Breakfast as give_breakfast;


fn main() {
    let mut my_breakfast = back_of_house::hosting::Breakfast::summer("Bread");
    my_breakfast.toast = String::from("tast");

    let test = give_breakfast::summer("Good");

    let order_type = back_of_house::hosting::Types::Veg;



}