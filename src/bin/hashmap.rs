use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut score = HashMap::new();
    score.insert(blue, 10);
    score.insert(yellow, 5);

    // println!("{}", blue); // it wont work because we haven't defined the key

    let team_name = String::from("blue");
    let scores = score.get(&team_name); // .get returns an Option<&V> coz what if the key is not there

    for (key, value) in score {
        println!("{}: {}", key, value);
    }

    // updating the value in the hashmap
    let mut hashmap = HashMap::new();
    hashmap.insert(String::from("key"), String::from("value"));
    hashmap.insert(String::from("key"), String::from("new value")); // this line overwrites the previous value

    hashmap
        .entry(String::from("yellow"))
        .or_insert(String::from("10")); // if the key is not present, it will insert the value 0
    hashmap
        .entry(String::from("yellow"))
        .or_insert(String::from("30")); // this will not change the value since the key is already present

    println!("{:?}", hashmap);
}
