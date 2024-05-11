mod enum_test;
mod array_test;
mod flow_test;
mod test_method;
mod test_generics;
mod test_trait;
mod test_trait_object;
mod test_vector;
mod test_map;

fn main() {
    // println!("Hello, world!");
    // let a = "a";
    // let b = "b";
    // let c = "c";
    // let arr = [a, b, c];
    // for x in arr.iter(){
    //     println!("{}", x)
    // }
    // let _x = 5;
    // let y = 1;
    test_tuple_struct();
}



struct Color(i32, i32, i32);
fn test_tuple_struct (){
    let black = Color(0, 0, 0);
    let red = Color(255, 0, 0);
    println!("{}", black.0);
}
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn test_user(){
    let user1 = User {
        email: String::from(""),
        username: String::from(""),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("user2"),
        ..user1
    };
    println!("{}", user2.email);
    println!("{}", user2.username);
    println!("{}", user1.active);
    // println!("{}", user1.username); ERROR  cause user1.username  is moved to user2
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
fn test_string(){
    let x = "hello";
    let mut s = String::from("helle ");
    s.push_str("dylan");
    println!("{}", s);
    s.push_str("!!!!");
    println!("{}", s);
    s.insert(5, '-');
    println!("{}", s);
    s.insert_str(5, "----");
    println!("{}", s);

    let s1 = String::from("hello1");
    let s2 = String::from("world2");
    let s3 = format!("{} {}", s1, s2);
    println!("{}", s3);
}
fn test_slice(){
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} AND  {}", hello, world);
}

fn test_tuple(){
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("The value of a is: {}", a);
    let b = a;
    println!("The value of b is: {}", b);
}






fn test_double_mut_evariable(){
    let mut x = 5;
    let y = &mut x;
    // let z = &mut x;
    // println!("{} {} {}", x, y, z);
}
fn test_immutable_borrow() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

fn change_string (str : &String){
    // str.push_str(" world");
}
fn change_mut_string (str : &mut String){
    str.push_str(" world");
}
fn test_borrowing() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    //assert_eq!(5, y);// ERROR
    assert_eq!(5, *y);
}

fn test_copy_transfer() {
    let x = 5;
    make_copy(x);
    println!("after make copy {}", x);

    let s = String::from("Hello");
    take_ownership(s);
    // println!("{}", s); ERROR her
}

fn make_copy(x: i32) {
    println!("{}", x);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn test_clone() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);
}

fn clear(text: &mut String) -> () {
    *text = String::from("");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn test_type() {
    let x = 2.0;
    let y: f32 = 3.0;
}

fn test_na_n() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("undefined")
    }
}

fn test_number() {
    let v: u16 = 38_u8 as u16;
    let arr = [
        42.0,
        42f32,
        42.0_f32
    ];
    let x = "22323";
    println!("size x [{}]", size_of_val(&x));
    println!("{:.10}", arr[0]);
    let y = 5;
    assert_eq!("i32".to_string(), type_of(&x));


    assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32);
}

use std::mem::size_of_val;

fn test_char() {
    let c1 = 'a';
    let c2 = 'b';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v.0, 2);


    println!("Success!")
}


fn print_char(c: char) {
    println!("{}", c);
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn test_range() {
    for i in 0..=5 {
        print!("{}", i);
    }
}

fn test_var() {
    let mut x = 5;
    println!("Value {}", x);
    x = 6;
    println!("Value {}", x);
}

fn test_shadowing() {
    let x = 10;
    let x = 15;
    println!("Value x {}", x);
    {
        let x = 20;
        println!("Value x {}", x);
    }
    println!("Value x{}", x);
}

struct Struct {
    e: i32,
}

