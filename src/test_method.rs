
pub struct Circle {
    radius: f64,
    x : f64,
    y : f64,
    //private field
}
#[derive(Debug)]
struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
}
impl Color {
    fn new() -> Color {
        Color::Rgb(0, 0, 0)
    }
    fn call (&self) -> String {
        match *self {
            Color::Rgb(r, g, b) => format!("rgb({},{},{})", r, g, b),
            Color::Hsv(h, s, v) => format!("hsv({},{},{})", h, s, v),
        }
    }


}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    pub fn new(radius: f64, x: f64, y: f64) -> Circle {
        Circle { radius, x, y }
    }
    pub  fn radius(&self) -> f64 {
        self.radius
    }
}
impl Circle{
    pub fn grow(&self, increment: f64) -> Circle {
        Circle { radius: self.radius + increment, x: self.x, y: self.y }
    }

}
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
#[test]
fn method_impl() {

    let c = Circle::new(2.0, 0.0, 0.0);
    println!("Area is {}", c.area());
    println!("Radius is {}", c.radius);

    let co = Color::new();
    println!("Color is {}", co.call());

}