mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {

    pub struct Breakfast {
        pub bread: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer_menu(bread: &str) -> Breakfast {
            Breakfast {
                bread: String::from(bread),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative
    front_of_house::hosting::seat_at_table();

    let mut meal = back_of_house::Breakfast::summer_menu("Rye");
    meal.bread = String::from("Wheat"); // Change the bread we want
    println!("I'd like {} bread please", meal.bread);
}