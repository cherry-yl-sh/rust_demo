#[test]
fn enum_test() {
    let c1 = Card::Club(1);
    let c2 = Card::Spades(2);

    let m1 = Message::Quit;
    let m2 = Message::Move{x: 1, y: 2};
    let m3 = Message::Write("Hello".to_string());

    let some_number = Some(5);
    let no_number: Option<i32> = None;
}
enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Card {
    Club(u8),
    Spades(u8),
}