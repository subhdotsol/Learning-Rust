use core::panic;

fn main() {
    // panic!("This is a panic message!"); // panic! macro triggers a panic in the current thread and the program will terminate

    // second example of using rust_backtrace
    a();

    // use this command to back trace
    //  RUST_BACKTRACE=1 crun error_handling.rs  --> in my case
    //  RUST_BACKTRACE=1 cargo run error_handling.rs  --> in your case
}

fn a() {
    b();
}

fn b() {
    c(21);
}

fn c(x: i32) {
    if x == 22 {
        panic!("Don't pass 22 to this function!");
    }
}

// go to error_handling_ex1.rs to see the first example
