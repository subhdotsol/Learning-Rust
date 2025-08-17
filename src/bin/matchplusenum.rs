fn main() {
    // Define an Option<i32> with the value Some(5)
    let five = Some(5);

    // Call plus_one function with `five` and store result in `six`
    let six = plus_one(five);

    // Print the result; {:?} is used to print an Option with Debug formatting
    println!("The six is {:?}", six);
}

// Define a function that takes an Option<i32> and returns an Option<i32>
fn plus_one(x: Option<i32>) -> Option<i32> {
    // Use match to handle both Some and None variants of Option
    match x {
        // If x is Some(i), return Some(i + 1)
        Some(i) => Some(i + 1), // we wrap the result in Some because the function returns Option<i32>

        // If x is None (no value), just return None
        None => None, // handle the None case explicitly
    }
}
