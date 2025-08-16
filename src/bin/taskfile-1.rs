// this file consist of 10 assignment you can build while you have learnt rust till functions

#![allow(dead_code)]

use std::io;

fn main() {
    // greeting();
    // simple_calculator();
    // even_or_odd();
    // temperature_converter();
    // positive_negative_zero();
    // word_len();
    // simple_interest_calculator();
    // number_guess();
    // grade_calculator();
    swap_numbers()
}

fn greeting() {
    let mut name = String::new();
    println!("Enter your name : ");
    io::stdin()
        .read_line(&mut name)
        .expect("Error while reading line : ");
    println!("Hello {} , Welcome to rust ...", name.trim());
}

fn simple_calculator() {
    let mut a = String::new();
    let mut b = String::new();
    let mut op = String::new();

    println!("Enter the value of a : ");
    io::stdin().read_line(&mut a).unwrap();

    println!("Enter the value of b : ");
    io::stdin().read_line(&mut b).unwrap();

    println!("Enter the operation to perform : ");
    io::stdin().read_line(&mut op).unwrap();

    let a: f64 = a.trim().parse().unwrap();
    let b: f64 = b.trim().parse().unwrap();
    let op = op.trim();

    if op == "+" {
        println!("Result is : {}", a + b);
    } else if op == "-" {
        println!("Result is : {}", a - b);
    } else if op == "*" {
        println!("Result is : {}", a + b);
    } else {
        println!("Invalid operation !!!")
    }
}

fn even_or_odd() {
    let mut number = String::new();
    println!("enter the number :");
    io::stdin().read_line(&mut number).unwrap();

    let number: i32 = number.trim().parse().unwrap();

    if number % 2 == 0 {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }
}

fn temperature_converter() {
    let mut celsius = String::new();
    println!("Enter the celsius : ");
    io::stdin().read_line(&mut celsius).unwrap();
    let celsius: f64 = celsius.trim().parse().unwrap();
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{}°C = {}°F", celsius, fahrenheit);
}

fn positive_negative_zero() {
    let mut num = String::new();
    println!("enter the number : ");
    io::stdin().read_line(&mut num).unwrap();
    let num: i32 = num.trim().parse().unwrap();

    if num > 0 {
        println!("The number is positive");
    } else if num < 0 {
        println!("the number is negative");
    } else {
        println!("Number is zero");
    }
}

fn word_len() {
    let mut word = String::new();
    println!("Enter the word : ");
    io::stdin().read_line(&mut word).unwrap();
    println!("The lenght of the word is  : {}", word.trim().len());
}

fn simple_interest_calculator() {
    let mut principle = String::new();
    let mut rate = String::new();
    let mut time = String::new();

    println!("Enter the principle : ");
    io::stdin().read_line(&mut principle).unwrap();

    println!("Enter the rate : ");
    io::stdin().read_line(&mut rate).unwrap();

    println!("Enter the time : ");
    io::stdin().read_line(&mut time).unwrap();

    let principle: f64 = principle.trim().parse().unwrap();
    let rate: f64 = rate.trim().parse().unwrap();
    let time: f64 = time.trim().parse().unwrap();

    let simple_interest = (principle * rate * time) / 100.0;
    println!("The simple interest is : {} ", simple_interest);
}

// number guessing no loop
fn number_guess() {
    let secret = 7;
    let mut guess = String::new();
    println!("Enter your guess");
    io::stdin().read_line(&mut guess).unwrap();
    let guess: i32 = guess.trim().parse().unwrap();

    if guess < secret {
        println!("Too low ");
    } else if guess > secret {
        println!("too high");
    } else {
        println!("congrats you guessed correct");
    }
}

// grade_calculator
fn grade_calculator() {
    let mut grade = String::new();
    println!("Enter your marks");
    io::stdin().read_line(&mut grade).unwrap();
    let grade: i32 = grade.trim().parse().unwrap();

    if grade >= 90 {
        println!("Grade A");
    } else if grade >= 75 {
        println!("Grade B");
    } else if grade >= 50 {
        println!("Grade C");
    } else {
        println!("fail");
    }
}

fn swap_numbers() {
    let mut a = String::new();
    let mut b = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut a).unwrap();

    println!("Enter second number:");
    io::stdin().read_line(&mut b).unwrap();

    let mut a: i32 = a.trim().parse().unwrap();
    let mut b: i32 = b.trim().parse().unwrap();

    println!("Before Swap: a = {}, b = {}", a, b);

    a = a + b;
    b = a - b;
    a = a - b;

    println!("After Swap: a = {}, b = {}", a, b);
}
