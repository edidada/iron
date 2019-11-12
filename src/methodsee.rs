
enum Direction {
    East,
    West,
    North,
    South,
}
pub fn testenum() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}

fn test_decly(){
    let x: i32 = 5;
    println!("{}", x.to_string());
}

pub fn testprint() {
    println!("hello, world");
    testprints();
}

fn testprints() {
    let rust: &str = "Rust";
    println!("Hello, {}!", rust);

    test_decly();
}


