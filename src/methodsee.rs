use std::collections::HashMap;

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
    map_insert();
    map_new();
    map_iterator();
}

fn map_insert(){
    let mut scores  = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    println!("{:?}", scores)
}

fn map_new(){
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores:HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores)
}

fn map_iterator(){
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    println!("{:?}", scores);

    println!("get---->{:?}", scores.get("blue"));

    for (k, v) in & scores{
        println!("{}:{}", k, v)
    }
}

fn map_update(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);//当插入一个键时，如果已经有这个键了就直接覆盖，如果没有就创建键

    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(25);
    scores.entry(String::from("Blue")).or_insert(25);//只在键没有对应值时插入
    println!("{:?}", scores);
}
