fn main() {
    println!("Hello, rust world!");

    // numeric dataypes (int, uint, float)
    let x: i32 = 5;
    let y: u8 = 100;
    let z: f64 = 1000.023;

    println!("x: {}, y: {}, z: {}", x, y, z);

    // boolean datatypes
    let is_male: bool = true;
    let is_above_18: bool = true;

    if is_male {
        println!("You are a male");
    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        println!("you are a legal male");
    }

    // string datatype
    let greeting: String = String::from("hello world");
    let greeting2: &str = "hello rust";

    println!("{}", greeting);
    println!("{}", greeting2);

    let ind: usize = 1;
    let char1: Option<char> = greeting.chars().nth(ind);

    println!("{}", char1.unwrap()); // not a good way

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No  charcater at index {}", ind),
    } // advisable method
}
