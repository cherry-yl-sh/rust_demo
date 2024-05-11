use std::fmt::Display;
use std::ops::Add;
use std::process::Output;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    title: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.title)
    }
}

pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn some_function<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Display,
{
    0
}


pub struct Twitter {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Twitter {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[derive(Debug)]
struct Point<T: Add<T, Output=T>> {
    x: T,
    y: T,
}

impl<T: Add<T, Output=T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[test]
fn test_point() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
}

#[test]
fn test_notify() {
    let s = 3.to_string();

    let a = 19;
    let b: u16 = 100;
    let b_str = b.to_string();
}

struct Sheep {
    naked: bool,
    name: String,
}

trait Animal {
    fn new(name: String) -> Self;
    fn name(&self) -> String;
    fn noise(&self) -> String;
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // `Sheep` 结构体上定义的方法可以调用 `Sheep` 所实现的特征的方法
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: String) -> Self {
        Sheep { naked: false, name }
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn noise(&self) -> String {
        if self.is_naked() {
            "baaaaah".to_string()
        } else {
            "baaaaah".to_string()
        }
    }
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

#[test]
fn test_sheep() {
    let mut dolly: Sheep = Animal::new("Dolly".to_string());
    dolly.talk();
}

