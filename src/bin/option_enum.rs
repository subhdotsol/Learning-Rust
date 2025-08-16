// Option<T> enum is one of the most commonly used types in Rust, and understanding where and why to use it will make you a much better Rust developer.

// what is this ?
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let nums = vec![1, 3, 7, 4, 5];

    match find_even(&nums) {
        Some(even) => println!("Found even number: {}", even),
        None => println!("No even number found"),
    }
}

fn find_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}

// usecases of Option enum :

// Accessing elements by index in a collection.
// Searching for a value that may not exist.
// Parsing user or external input safely.
// Reading a character at a specific string position.
// Returning a value conditionally from a function.
// Representing optional fields in a struct.
// Looking up values in a map-like data structure.
// Removing keys from a map and handling missing ones.
// Handling the next element from an iterator.
// Transforming values using chaining methods like .map().
// Safely dividing numbers with possible zero divisor.
// Using pattern matching to handle missing values.
// Avoiding panics from invalid operations.
// Providing default values when none is present
// Holding values that are not initialized yet.
// Allowing optional configuration parameters.
// Controlling behavior with optional flags.
// Skipping logic when data is missing
// Handling fallible computations without errors.
// Delaying computation until value is needed.
