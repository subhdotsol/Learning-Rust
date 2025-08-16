// rules of mutability and immutibility ::
// -> one thing to keep in mind is that you can read a same variable at multiple places in a program, but you can't write to it.
// ✅ One mutable reference (&mut) at a time.
// ✅ Can have multiple immutable references (&), but no &mut while & exists.
// ✅ Mutable reference goes out of scope before a new one is created.

fn main() {
    let name = String::from("Subhajit Chaudhury");

    let first_name = first_name(&name);
    let another_first_name = another_first_name(&name);
    println!("The first name is {}", first_name);
    println!("The another first name is {}", another_first_name);

    // now writing to a string making it mutable

    let mut test = String::from("Neha Sharma");
    update_name(&mut test);
    println!("The updated name is {}", test); // the scope of this mustable reference ends here

    // intentinal error below ...

    let mut another_test = String::from("Testing two mutable ref in same scope"); // we got two mutable reference because the first one ended on line 16
    let r1 = &mut another_test;
    // let r2 = &mut another_test;     // uncomment to see the error
    // println!("{}, {}", r1, r2);
}

fn first_name(name: &String) -> &str {
    match name.find(' ') {
        Some(index) => &name[..index],
        None => name,
    }
}

fn another_first_name(name: &String) -> &str {
    for (index, char) in name.char_indices() {
        if char == ' ' {
            return &name[..index];
        }
    }
    &name[..] // if no space found return the whole string
}

fn update_name(str: &mut String) -> &mut String {
    str.push_str(" is a Rustacean");
    str
}
