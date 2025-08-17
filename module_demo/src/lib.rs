// use rand::{Rng , CryptoRng , ErrorKind};   // nested path
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn sit_on_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn server_order() {}

        pub fn recieve_payment() {}
    }
}

// use keyword in rust

use self::front_of_house::hosting::add_to_waitlist; // specifying the absolute path
// use crate::front_of_house::hosting::add_to_waitlist;  // specifying the relative path

// if i want to use these
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // this will give an error cos functions and mod is not pub its private by default coz of rust privacy rules
    add_to_waitlist(); // -> This will work because we used the use keyword and now the function is in our scope already

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// name conflicting issues

// use std::io;  // bring the parent module or rename one of them like done below
// use std::fmt;

// rename is done like this
// use std::fmt::Result;
// use std::io::Result as ioResult;

// fn function()-> fmt::Result {

// }

// fn function2()-> io::Result {

// }
