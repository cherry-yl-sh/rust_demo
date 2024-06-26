#[test]
fn test() {
    let a = "hello";
    let b = "world";
    let result = longest(a, b);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}