// key takeaways for references :
// & sends the memory address of the value — a reference (similar to a pointer in C/C++).
// It does NOT copy or move the actual value.
// The function receiving &T can read from that memory location without taking ownership.

// You never deal with raw pointers directly unless using unsafe.
// References are guaranteed to be valid and safe by Rust’s borrow checker.
// Passing by reference is efficient because you avoid copying large data.

fn main() {
    let mut name = String::from("Subhajit Chaudhury");
    let new_name = &name;
    println!("The new name address is {:p}", &new_name);
    print_name(&name);
    add_name(&mut name);
    print_name(&name);
}

fn print_name(name: &String) {
    println!("The name is {}", name);
}

fn add_name(name: &mut String) -> &mut String {
    name.push_str(" is a Rustacean");
    name
}
