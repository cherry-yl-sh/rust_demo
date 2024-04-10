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
    test_shadowing()
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