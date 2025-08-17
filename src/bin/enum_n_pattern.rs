// structs and enums are the building block of creating new types in Rust.
// Option enum is a special enum that has two variants, Some and None.

// enum allows to enumerate a list of variants

#![allow(unused_variables)]
#![allow(dead_code)]
enum IpAddrKind {
    // it only able to stroe the type of the address for storign the actual address you need a struct
    V4(String),
    V6(String),
}

enum Message {
    // enum can store wide variety of types we could have created seperate structs for each type but it would look ugly
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // let four = IpAddrKind::V4; // variants are namespaced under their identifiers
    // let six = IpAddrKind::V6;

    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4, // it accepts enum bout founds enum constructor
    //     address: String::from("127.0.0.1"),
    // };

    let localhost = IpAddrKind::V4(String::from("127.0.0.1")); // the will start complaining if you change the enum v4 from string to u32
}

fn route(ip_kind: IpAddrKind) {}
