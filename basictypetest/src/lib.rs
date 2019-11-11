#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn int_test() {
        let mut a : i64 =-4;
        assert_eq!(2 - 6, a);

        let x = 2.0; // f64

        let y: f32 = 3.0; // f32

        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;

        // remainder
        let remainder = 43 % 5;

        let t = true;

        let f: bool = false; // with explicit type annotation

        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';


        let tup: (i32, f64, u8) = (500, 6.4, 1);//The Tuple Type
    }

    #[test]
    fn tuple_test() {
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;
    }

    #[test]
    fn array_test() {
        let a = [1, 2, 3, 4, 5];


        let first = a[0];
        let second = a[1];

        let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];
        let bb: [i32; 5] = [1, 2, 3, 4, 5];

        let cc = [3; 5];
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

//        if number {//error[E0308]
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
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {}", result);
    }

    #[test]
    fn while_test(){
        let mut number = 3;

        while number != 0 {
            println!("{}!", number);

            number -= 1;
        }

        println!("LIFTOFF!!!");
    }

    #[test]
    fn while_test_2(){
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

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
