
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

fn test(){
    let x: i32 = 5;

}

pub fn testprint() {
    println!("hello, world");
}

fn testprints() {
    let rust = "Rust";
    println!("Hello, {}!", rust);
}

fn testlet() {
    let a1 = 5;
    let a2:i32 = 5;
    assert_eq!(a1, a2);
    //let 绑定 整数变量默认类型推断是 i32

    let b1:u32 = 5;
    //assert_eq!(a1, b1);
    //去掉上面的注释会报错，因为类型不匹配
    //errer: mismatched types
}

//rust 在声明变量时，在变量前面加入 mut 关键字，变量就会成为可变绑定的变量。
fn testa() {
    let mut a: f64 = 1.0;
    let b = 2.0f32;

    //改变 a 的绑定
    a = 2.0;
    println!("{:?}", a);

    //重新绑定为不可变
    let a = a;

    //不能赋值
    //a = 3.0;

    //类型不匹配
    //assert_eq!(a, b);
}

fn testlet2() {
    let (a, mut b): (bool,bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    //a 不可变绑定
    //a = false;

    //b 可变绑定
    b = true;
    assert_eq!(a, b);
}

fn tests(){

    // boolean type
    let t = true;
    let f: bool = false;

// char type
    let c = 'c';

// numeric types
    let x = 42;
    let y: u32 = 123_456;
    let z: f64 = 1.23e+2;
    let zero = z.abs_sub(123.4);
    let bin = 0b1111_0000;
    let oct = 0o7320_1546;
    let hex = 0x7320_1511;

// string types
    let str = "Hello, world!";
    let mut string = str.to_string();

// arrays and slices
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4];
    let mut ten_zeros: [i64; 10] = [0; 10];

// tuples
    let tuple: (i32, &str) = (50, "hello");
    let (fifty, _) = tuple;
    let hello = tuple.1;

// raw pointers
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };

    // functions
    fn foo(x: i32) -> i32 { x }
    let bar: fn(i32) -> i32 = foo;
// explicit conversion
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;

    // type aliases
    type NanoSecond = u64;
    type Point = (u8, u8);
}


fn testarray() {
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);

    // This loop prints: 0 1 2
    for x in &array {
        println!("{} ", x);
    }

    //创建空Vec
    let v: Vec<i32> = Vec::new();
//使用宏创建空Vec
    let v: Vec<i32> = vec![];
//创建包含5个元素的Vec
    let v = vec![1, 2, 3, 4, 5];
//创建十个零
    let v = vec![0; 10];
//创建可变的Vec，并压入元素3
    let mut v = vec![1, 2];
    v.push(3);
//创建拥有两个元素的Vec，并弹出一个元素
    let mut v = vec![1, 2];
    let two = v.pop();
//创建包含三个元素的可变Vec，并索引一个值和修改一个值
    let mut v = vec![1, 2, 3];
    let three = v[2];
    v[1] = v[1] + 5;
}

fn teststr(){
    // 字符串字面值
    let hello = "Hello, world!";

// 附带显式类型标识
    let hello: &'static str = "Hello, world!";

    // 创建一个空的字符串
    let mut s = String::new();
// 从 `&str` 类型转化成 `String` 类型
    let mut hello = String::from("Hello, ");
// 压入字符和压入字符串切片
    hello.push('w');
    hello.push_str("orld!");

// 弹出字符。
    let mut s = String::from("foo");
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('f'));
    assert_eq!(s.pop(), None);
}

