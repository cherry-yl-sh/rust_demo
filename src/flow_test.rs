#[test]
fn flow() {
    let list = [1, 2, 3];
    for i in list.iter() {
        println!("{}", i);
    }
    for item in list {
        println!("{}", item);
    }
    for i in 0..list.len() {
        println!("{}", list[i]);
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,

}

enum Action {
    Say(String, String),
    Move { x: i32, y: i32 },
    Eat(String),
}

#[test]
fn test_match() {
    let dir = Direction::Up;
    match dir {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
    }

    let actions =[
        Action::Say("dylan".to_string(), "Hello".to_string()),
        Action::Move { x: 1, y: 2 },
        Action::Eat("apple".to_string()),
    ];
    for action in actions.iter(){
        match action {
            Action::Say(h, s) => println!("{} say :[{}]", h, s),
            Action::Move { x, y } => println!("Move to x = {}, y = {}", x, y),
            Action::Eat(s) => println!("Eat {}", s),
        }
    }

}