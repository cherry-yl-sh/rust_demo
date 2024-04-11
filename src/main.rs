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
    test_char()
}
fn test_type(){
    let x = 2.0;
    let y :f32 = 3.0;
}
fn test_NaN(){
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("undefined")
    }
}
fn test_number(){
    let v: u16 = 38_u8 as u16;
    let arr = [
        42.0,
        42f32,
        42.0_f32
    ];
    let x = "22323";
    println!("size x [{}]",std::mem::size_of_val(&x));
    println!("{:.10}",arr[0]);
    let y = 5;
    assert_eq!("i32".to_string(), type_of(&x));


    assert!(0.1_f32+0.2_f32==0.3_f32);
}
use std::mem::size_of_val;
fn test_char(){
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}
fn print_char(c : char) {
    println!("{}", c);
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
fn test_range(){
    for i in 0..=5{
        print!("{}",i);
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
    println!("Value x{}",x);
}

struct Struct {
    e: i32,
}