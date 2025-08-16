// rules of rust ownership :
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    let s = String::from("Hello World");
    let name = String::from("Subhajit Chaudhury");

    let len = get_length(s);
    println!("The length is {:?} : ", len);
    // println!("The length of '{}' is {:?}.", s, len); // can't use s here, ownership moved to function

    let (name, len) = another_get_length(name);
    println!("The length of '{}' is {:?}.", name, len);
}

fn get_length(s: String) -> usize {
    s.len()
}

fn another_get_length(name: String) -> (String, usize) {
    let len = name.len(); // calculate the lenght first and the return
    (name, len)
}
