use std::fmt::Display;

// Define a generic struct Pair<T>
struct Pair<T> {
    x: T,
    y: T,
}

// General implementation for all Pair<T>
impl<T> Pair<T> {
    // Constructor method to create a new Pair
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Implementation only available if T implements both Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    // Compare and display the larger member
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    // Create a pair of integers
    let p1 = Pair::new(10, 20);
    p1.cmp_display();

    // Create a pair of floating-point numbers
    let p2 = Pair::new(3.5, 2.1);
    p2.cmp_display();

    // Create a pair of strings
    let p3 = Pair::new(String::from("apple"), String::from("banana"));
    p3.cmp_display();
}
