fn main() {
    // Its a infinite loop below
    // loop {
    //     println!("Hello World");
    // }

    for i in 0..5 {
        println!("{}", i);
    }

    let name = String::from("Hello world");
    let first_word = get_first_word(name);

    println!("{}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut str: String = String::from("");

    for char in sentence.chars() {
        str.push_str(char.to_string().as_str());

        if char == ' ' {
            break;
        }
    }

    return str;
}
