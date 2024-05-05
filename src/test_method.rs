
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
#[test]
fn method_impl() {

    let c = Circle::new(2.0, 0.0, 0.0);
    println!("Area is {}", c.area());
    println!("Radius is {}", c.radius);

}