#[test]
fn test_vector() {
    let mut v: Vec<i32> = Vec::with_capacity(10);
    println!("v.len() = {}, v.capacity() = {}", v.len(), v.capacity());
    v.push(5);
    v.reserve(100);
    println!("v.len() = {}, v.capacity() = {}", v.len(), v.capacity());

    v.push(6);

    let arr = vec![1, 2, 3];

}