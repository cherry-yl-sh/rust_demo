fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn display_array<T: std::fmt::Debug, const N: usize>(arr :[T ;N]){

    for i in 0..N {
        println!("arr[{}] = {:?}", i, arr[i]);
    }
}
#[test]
fn test_generics() {
    let x = add(1, 2);
    let a_point = Point { x: 1, y: 2 };
    println!("x [{}]", x);
    println!("a_point.x [{}]", a_point.x());
    // let b_point = Point { x: 1, y: 2.1 };

    let arr_a = [1, 2, 3, 4, 5];
    display_array(arr_a);
    let arr_b = ['a', 'b', 'c', 'd', 'e'];
    display_array(arr_b);
}