mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { get_waitlist() }

        fn get_waitlist() {println!("get waitlist done")}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
        tea: bool
    }

    impl Breakfast {
        pub fn make_breakfast(
            toast: String, 
            seasonal_fruit: String, 
            tea: bool) -> Breakfast {
                Breakfast {toast, seasonal_fruit, tea}
        }

        pub fn take_a_bite(self) -> () {
            println!("Mmmm, toast with {} tastes good", self.seasonal_fruit);
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // //absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // //relative path
    // front_of_house::hosting::add_to_waitlist();

    // let new_pc = front_of_house::Breakfast::make_breakfast(
    //     String::from("Rye bread"), 
    //     String::from("Ananas"), 
    //     true);
    
    // new_pc.take_a_bite();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    hosting::add_to_waitlist();
}