// vector is a collection of elements of same type

#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut v: Vec<i32> = Vec::new();
    v.push(1); // way to push elements in vector
    v.push(2);
    v.push(3);

    println!("{:?}", v);
    {
        let vec2 = vec![1, 2, 3, 4, 5]; // macro to create vector
    }

    // accessing elements
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let third = &my_vec[4];
    // let third = &my_vec[20];  // out of bound error
    println!("third number is: {}", third);

    // so if we dont want our code to crash if we get such out of bound error then rust provides way to handle it gracefully
    match my_vec.get(20) {
        Some(third) => {
            println!("third number is: {}", third);
        }
        None => {
            println!("out of bound error"); // now it will not crash on out of bount error 
        }
    }

    // iterating over vector
    for i in &my_vec {
        println!("{}", i);
    }

    // what if we want to iterate and update the vector
    for i in &mut my_vec {
        *i = *i * 2; // * is dereference operator for accessing the underline value of the reference 
        println!("{}", i);
    }

    // what if we want to have different type of data in vector
    // storing enum variants inside the vector
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(1),
        SpreadSheetCell::Float(2.0),
        SpreadSheetCell::Text(String::from("hello")),
    ];

    match &row[2] {
        SpreadSheetCell::Int(i) => {
            println!("i is {}", i);
        }
        _ => {
            println!("not an int");
        }
    }
}
