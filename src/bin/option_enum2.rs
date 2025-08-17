#![allow(unused_variables)]

fn main() {
    let some_number = Some(5);
    let some_string = Some(String::from("Hello, world!"));
    let none_string: Option<String> = None;

    let x: i8 = 8;
    let y: Option<i8> = None;
    // let y = Some(5);    // this works

    // let sum = x + y;    // this complains you cant add an option type to an integer
    // so lets say we want to use y if its present if not it default to 5
    let sum = x + y.unwrap_or(5);
    println!("The sum is {}", sum);
}
