// Strings are pretty damn complicated
// Strings are stored as a collection of UTF-8 encoded bytes

#![allow(unused)]
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut s = String::from("Hello World");
    println!("{}", s);

    let str = String::new(); // creating a new empty string
    let s2 = "Hello World"; // creating a string from a string literal
    let s2 = s2.to_string(); // converting a string literal to a String

    // appeding to a string
    s.push_str(" cutie");
    println!("{}", s);

    let my_string = String::from("Hello");
    let my_string2 = String::from("World");
    // let s3 = my_string + &my_string2;   // we are moving ownership of my_string to s3

    // we can also concatenate strings using format! macro
    let s3 = format!("{} {}", my_string, my_string2);
    println!("{}", s3);

    // we cant access the string like we do in high level languages like another_string[0]
    let another_string = String::from("नमस्ते");
    // It can be represented in three ways
    // Bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // scalar value
    // ['न', 'म', 'स', '्', 'त', 'े']
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // Grapheme clusters
    // ["न", "म", "स्", "ते"]
    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    }
}
