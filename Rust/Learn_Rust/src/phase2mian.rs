use std::fs;
use std::io;
fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
struct User {
    name: String,
    email: String,
    is_active: bool,
}
struct Rectangle {
    width: f64,
    height: f64,
}
impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
pub fn moin() {
    println!("------------------------------\n Ownership \n------------------------------");
    let s1 = "hello";
    println!("{s1} world");
    let s2 = s1;
    println!("{s2} world");
    // println!("{s1}"); Error s1 no longer valid

    let x = 5;
    let y = x;
    println!("{x}   ll{y}");
    // copy trait
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED into s2
    // println!("{s1}"); // ERROR: s1 no longer valid
    println!("{s2}"); // OK

    let x = 5;
    let y = x; // Copy trait — both valid
    println!("{x} and {y}");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{s1} and {s2}"); // Both valid

    let s1 = "lolo";
    let s2 = s1;
    println!("{s1}      {s2}");

    println!("-----------------------------\n Refrencing \n-----------------------------");
    let s = String::from("hello");
    let len = length(&s);
    println!("{s} has {len} chars"); // s still valid!

    let mut s = String::from("hello");
    append(&mut s);
    println!("{s}"); // "hello, world"
    println!("-----------------------------\n Struct \n----------------------------");
    let user = User {
        name: String::from("Goga G"),
        email: String::from("GogaG@on.top"),
        is_active: true,
    };

    println!("{}--{}", user.name, user.email);
    print!("{}", user.is_active);
    let rect = Rectangle::new(4.0, 6.0);
    println!("Area: {}", rect.area());
    println!("Is Square: {}", rect.is_square());
    println!("-----------------------------\n Enums \n----------------------------");
    process(Message::Quit);
    process(Message::Move { x: 3, y: 5 });
    process(Message::Write(String::from("Goga G")));
    process(Message::Color(255, 255, 255));
    match divide(10.0, 2.0) {
        Some(result) => println!("Result: {result}"),
        None => println!("Division by zero!"),
    }
    println!(
        "------------------------------\n Exeption handeling \n------------------------------"
    );
    match read_file("hello.txt") {
        Ok(content) => println!("{content}"),
        Err(e) => println!("{e}"),
    };
    match read_username() {
        Ok(name) => println!("hello {name}"),
        Err(e) => eprintln!("Failed {e}"),
    }
    let n: i32 = "42".parse().expect("should be a valid number!");
    println!("{n}");
}

fn read_username() -> Result<String, io::Error> {
    let content = fs::read_to_string("username.txt")?;
    Ok(content.trim().to_string())
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}
fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to {x} ,{y}"),
        Message::Write(text) => println!("Write: {text}"),
        Message::Color(r, g, b) => println!("Color({r},{g},{b}"),
    }
}
fn append(s: &mut String) {
    s.push_str(", world");
}

fn length(s: &String) -> usize {
    s.len()
}
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}
