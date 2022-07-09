// Use the glob operator
use std::collections::*;

mod front_of_house;

mod back_of_house;

mod customer {
  use crate::front_of_house::hosting;
  use crate::back_of_house;
  pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
  }
}

