// generics are used in code deduction lets deduce some code
// comment the other steps back when not in use try to understand one by one

// step 1 :

// fn main() {
//     // first section generics with function arguments
//     // first section generics with function arguments
//     let num_list = vec![20, 43, 56, 2343];

//     let mut largest = num_list[0];
//     for num in num_list {
//         if num > largest {
//             largest = num;
//         };
//     }
//     println!("The largest number is {}", largest);
// }

// step 2 : we did the logic separately in a function

// fn main() {
//     let num_list = vec![20, 43, 56, 2343];

//     let largest = get_max(num_list);
//     println!("The largest number is {}", largest);
// }

// fn get_max(num_list: Vec<i32>) -> i32 {  // its still tied to concrete type of argument
//     let mut largest = num_list[0];
//     for num in num_list {
//         if num > largest {
//             largest = num;
//         };
//     }
//     largest
// }

// step 3 : what if we want to find the largest character (wrong way)
// fn main() {
//      // first section generics with function arguments
//     let num_list = vec![20, 43, 56, 2343];
//     let char_list = vec!['a', 'b', 'c', 'd']; // created a char list vec
//     let largest = get_max(num_list);
//     let max_char = max_char(char_list); // passing it to the newly created function
//     println!("The largest number is {}", largest);
//     println!("The largest character is {}", max_char);

//     fn get_max(num_list: Vec<i32>) -> i32 {
//         // its still tied to concrete type of argument
//         let mut largest = num_list[0];
//         for num in num_list {
//             if num > largest {
//                 largest = num;
//             };
//         }
//         largest
//     }
// }
// fn max_char(num_list: Vec<char>) -> char {
//     // we had to create a whole new function for CHAR hence code repetition
//     // its still tied to concrete type of argument
//     let mut largest = num_list[0];
//     for num in num_list {
//         if num > largest {
//             largest = num;
//         };
//     }
//     largest
// }

// step 4 : lets use generics to solve this problem

// fn main() {
// first section generics with function arguments
//     let num_list = vec![20, 43, 56, 2343];
//     let char_list = vec!['a', 'b', 'c', 'd']; // created a char list vec
//     let largest = get_max(num_list);
//     let max_char = get_max(char_list); // passing it to the newly created function
//     println!("The largest number is {}", largest);
//     println!("The largest character is {}", max_char);
// }

// fn get_max<T>(num_list: Vec<T>) -> T {
//     // using generics to solve the problem now its not tied to any type
//     // its still tied to concrete type of argument
//     let mut largest = num_list[0];
//     for num in num_list {
//         if num > largest {
//             // here is a issue for solving that we need to use traits
//             largest = num;
//         };
//     }
//     largest
// }

// step 5 : using traits to solve the issue
// fn main() {
//     // first section generics with function arguments

//     let num_list = vec![20, 43, 56, 2343];
//     let char_list = vec!['a', 'b', 'c', 'd']; // created a char list vec
//     let largest = get_max(num_list);
//     let max_char = get_max(char_list); // passing it to the newly created function
//     println!("The largest number is {}", largest);
//     println!("The largest character is {}", max_char);
// }

// fn get_max<T: PartialOrd + Copy>(num_list: Vec<T>) -> T {
//     // we will use traits later
//     // using generics to solve the problem now its not tied to any type
//     // its still tied to concrete type of argument
//     let mut largest = num_list[0];
//     for num in num_list {
//         if num > largest {
//             // here is a issue for solving that we need to use traits
//             largest = num;
//         };
//     }
//     largest
// }

// step 6 : using generics with structs
// struct Point<T> {
//     // for different types we can use <T, U> etc
//     x: T,
//     y: T,
// }

// fn main() {
//     let p = Point { x: 10, y: 20 };
//     println!("Point p: ({}, {})", p.x, p.y);
// }

// step 7 : using generics with enums

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// step 8 : using generics in method definitions
struct Point<T> {
    x: T,
    y: T,
}

struct AnotherPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T, U> AnotherPoint<T, U> {
    fn mixup<V, W>(self, other: AnotherPoint<V, W>) -> AnotherPoint<T, W> {
        AnotherPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // Example using Point
    let p = Point::new(10, 20);
    println!("Point p: ({}, {})", p.x, p.y);

    // Example using AnotherPoint
    let p3 = AnotherPoint { x: 5, y: "Hello" };
    let p5 = AnotherPoint { x: 10.5, y: 'Z' };

    // Mix two different AnotherPoint instances
    let p4 = p3.mixup(p5);

    println!("AnotherPoint p4: ({}, {})", p4.x, p4.y);
}
