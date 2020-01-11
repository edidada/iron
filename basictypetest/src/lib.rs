pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    fn test_string_copy(){
        let x = 5;
        let y = x;

        println!("x: {}, y: {}!", x,y);

        let s1 = String::from("hello");
        let s2 = s1;

//        println!("{}, world!", s1);//^^ value borrowed here after move

//        = note: move occurs because `s1` has type `std::string::String`, which doesnot implement the `Copy` trait
    }

    #[test]
    fn test_string_clone(){
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }


    #[test]
    fn mains() {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
        // ... and so is no longer valid here

        let x = 5;                      // x comes into scope

        makes_copy(x);                  // x would move into the function,
        // but i32 is Copy, so it’s okay to still
        // use x afterward

    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    fn test_integer() {
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
    fn test_mut() {
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

    fn test_boolean() {
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
        let hello : &str = "Hello, world!";

// 附带显式类型标识
        let hello: &'static str = "Hello, world!";

        // 创建一个空的字符串
        let mut s : String = String::new();
// 从 `&str` 类型转化成 `String` 类型
        let mut hello = String::from("Hello, ");
// 压入字符和压入字符串切片
        hello.push('w');
        hello.push_str("orld!");

// 弹出字符。
        let mut s : String = String::from("foo");
        assert_eq!(s.pop(), Some('o'));
        assert_eq!(s.pop(), Some('o'));
        assert_eq!(s.pop(), Some('f'));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn int_test() {
        let mut a : i64 =-4;
        assert_eq!(2 - 6, a);

        let x : f64 = 2.0; // f64

        let y: f32 = 3.0; // f32

        // addition
        let sum : i32 = 5 + 10;

        // subtraction
        let difference : f64 = 95.5 - 4.3;

        // multiplication
        let product : i32 = 4 * 30;

        // division
        let quotient : f64 = 56.7 / 32.2;

        // remainder
        let remainder: i32 = 43 % 5;

        let t:bool = true;

        let f: bool = false; // with explicit type annotation

        let c:char = 'z';
        let z:char = 'ℤ';
        let heart_eyed_cat:char = '😻';


        let tup: (i32, f64, u8) = (500, 6.4, 1);//The Tuple Type
    }

    #[test]
    fn tuple_test() {
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred : i32 = x.0;

        let six_point_four:f64 = x.1;

        let one:u8 = x.2;

//        println!(five_hundred);// error: format argument must be a string literal

//        println!(five_hundred.to_string());
        println!("{}", five_hundred.to_string());//为什么下面可以？
        println!("Hello, world!");
    }

    #[test]
    fn array_test() {
//        let a = [1, 2, 3, 4, 5];
        let a : [i32;5] = [1, 2, 3, 4, 5];


        let first :i32 = a[0];
        let second :i32 = a[1];

        let months: [&str;12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];//mismatched types [E0308] expected `[&str; 11]`, found `[&str; 12]`
        let bb: [i32; 5] = [1, 2, 3, 4, 5];

        let cc : [i32;5] = [3; 5];//;表示重复
    }

    #[test]
    fn call_function_test() {
        println!("Hello, world!");

        another_function();
        another_function_must_has_another_name(1);// to aviod redefine
        another_function_must_has_another_name_2(1,2);
    }

    fn another_function() {
        println!("Another function.");
    }

    fn another_function_must_has_another_name(x: i32) {
        println!("The value of x is: {}", x);
    }

    fn another_function_must_has_another_name_2(x: i32, y: i32) {
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
    }

    #[test]
    fn multy_statement(){
//        let x = (let y = 6);//has error
    }

    #[test]
    fn mys(){
        let x = 5;

//        let y = {
//            let x = 3;
//            x + 1
//        };//has error
//
//        println!("The value of y is: {}", y);
    }

    #[test]
    fn test_method_withreturnvalue(){
        let x = five();
        println!("{}",x);

        plus_one(1);
        plus_one(x);

        println!("{}",x);
    }

    fn five() -> i32 {
        5
    }

    fn plus_one(x: i32) -> i32 {
        x + 1;// has error
        x + 1
    }

    #[test]
    fn if_test(){
        let number = 3;//if do not define number varivale,cause error[E0425]
        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }

//        if number {//error[E0308]:println!("{}",x);
//            println!("number was three");
//        }

        if number != 0 {
            println!("number was something other than zero");
        }

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }

        let condition : bool = true;
        let number : i32 = if condition {
            5
        } else {
            6
        };

        println!("The value of number is: {}", number);

    }

    #[test]
    fn loop_test(){
        let mut counter: i32 = 0;

        let result : i32 = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {}", result);
    }

    #[test]
    fn while_test(){
        let mut number: i32 = 3;

        while number != 0 {
            println!("{}!", number);

            number -= 1;
        }

        println!("LIFTOFF!!!");
        println!("The number is {}", number);
    }

    #[test]
    fn while_test_2(){
        let a:[i32;5] = [10, 20, 30, 40, 50];
        let mut index: usize = 0;

        while index < 5 {
            println!("the value is: {}", a[index]);

            index += 1;
        }
    }

    #[test]
    fn while_with_iter(){
        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            println!("the value is: {}", element);
        }
    }

    #[test]
    fn for_rev(){
        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
    }

}
