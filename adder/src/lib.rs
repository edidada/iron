pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod adder {

    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }


    #[test]
    //#[should_panic]
    fn tesssssss() {
//        println!("Hello, world!");


// 通过闭包和函数分别实现自增。
// 译注：下面这行是使用函数的实现
fn  function            (i: i32) -> i32 { i + 1 }

        // 闭包是匿名的，这里我们将它们绑定到引用。
        // 类型标注和函数的一样，不过类型标注和使用 `{}` 来围住函数体都是可选的。
        // 这些匿名函数（nameless function）被赋值给合适地命名的变量。
        let closure_annotated = |i: i32| -> i32 { i + 1 };
        let closure_inferred  = |i     |          i + 1  ;

        // 译注：将闭包绑定到引用的说法可能不准。
        // 据[语言参考](https://doc.rust-lang.org/beta/reference/types.html#closure-types)
        // 闭包表达式产生的类型就是 “闭包类型”，不属于引用类型，而且确实无法对上面两个
        // `closure_xxx` 变量解引用。

        let i = 1;
        // 调用函数和闭包。
        println!("function: {}", function(i));
        println!("closure_annotated: {}", closure_annotated(i));
        println!("closure_inferred: {}", closure_inferred(i));

        // 没有参数的闭包，返回一个 `i32` 类型。
        // 返回类型是自动推导的。
        let one = || 1;
        println!("closure returning one: {}", one());


        assert!(true);
        assert_eq!(1+1, 2+0);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_vector(){
        let vs: Vec<i32> = Vec::new();

        let vv = vec![1, 2, 3];


        let mut aaaaaaaaaaaaaaa = Vec::new();

        aaaaaaaaaaaaaaa.push(5);
        aaaaaaaaaaaaaaa.push(6);
        aaaaaaaaaaaaaaa.push(7);
        aaaaaaaaaaaaaaa.push(8);



        let first = &vv[0];

//        vv.push(6);

        println!("The first element is: {}", first);
    }

    #[test]
    fn test_ve2(){
        let aaaaaaa = vec![1, 2, 3, 4, 5];

        let third: &i32 = &aaaaaaa[2];
        println!("The third element is {}", third);

        match aaaaaaa.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }


    }

    #[test]
    fn test_range(){
        let aaaaaaaaa = vec![1, 2, 3, 4, 5];

//        let does_not_exist : &i32 = &aaaaaaaaa[100];
//        let does_not_exist :Option<&i32> = aaaaaaaaa.get(100);



    }
}
