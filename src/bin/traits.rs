use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// create a trait and implement trait for both structs
pub trait Summary {
    fn summary(&self) -> String;
    // we can also have default implementations
    // fn summary(&self) -> String {
    //     String::from("No summary available")
    // }
}

impl Summary for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("subhajit"),
        content: String::from("Hello world!"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Rust is awesome!"),
        author: String::from("subhajit"),
        content: String::from("Rust is awesome!"),
    };

    println!("Summary of the tweet: {}", tweet.summary());
    println!("Summary of the article: {}", article.summary());

    // Using the notify function with a trait bound
    notify(&tweet);
}

// section 2 : trait bounds

// syntactical sugar for trait bounds

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summary());
// }

pub fn notify<T: Summary>(item: &T) {
    // this generic is restricted to types that implement the Summary trait
    println!("Breaking news! {}", item.summary());
}

// step 3 : multiple traits
pub fn something<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

// step 4 : using where clause for multiple traits
pub fn something_else<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0 // just a placeholder
}
