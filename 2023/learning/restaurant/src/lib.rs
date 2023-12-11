mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            super::test_restaurant()
        }

        fn seat_at_table() {}
    }

    mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    pub fn test_restaurant() {
        crate::front_of_house::hosting::add_to_waitlist();

        hosting::add_to_waitlist();

        serving::take_order();
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}