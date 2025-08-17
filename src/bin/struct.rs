//How to declare a struct ?

#![allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: String,
    gender: String,
}

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

// implementing the struct rect with an associated area function
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut user = Person {
        name: String::from("Subhajit Chaudhury"),
        age: 25,
        address: String::from("Bangalore"),
        gender: String::from("Male"),
    };

    println!("{:?}", user);

    // How to access the fields of as struct ?
    let name = user.name;
    println!("{}", name);
    // for updating any feild in the struct the entire struct must be mutable
    user.name = String::from("Subhajit Chaudhury is a Rustacean");
    println!("{:?}", user);

    let user2 = build_user(
        String::from("Subh"),
        25,
        String::from("Bangalore"),
        String::from("Male"),
    );
    println!("{:?}", user2);

    // lets create user 3 with its own name and age but everything else is same as user 2
    let user3 = Person {
        name: String::from("Subhajit"),
        age: 26,
        ..user2 // ..works like a spread operator
    };
    println!("{:?}", user3);

    // tuple struct
    let color = Color(255, 0, 0);
    println!("{:#?}", color);

    // implementing a struct so that we can use the associated functions of the struct
    let rect = Rect {
        width: 10,
        height: 20,
    };

    println!("Area of the rect is {}", rect.area());

    // using another function of the struct can_hold
    let rect2 = Rect {
        width: 5,
        height: 10,
    };

    println!("Can the rect hold the rect2 ? {}", rect.can_hold(&rect2));
}

// function to build a struct
fn build_user(name: String, age: u32, address: String, gender: String) -> Person {
    Person {
        name,
        age,
        address,
        gender,
    }
}
