//https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html

#[cfg(test)]
mod tests {

    #![allow(unused_variables)]
    #[test]
    fn it_works() {
        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שָׁלוֹם");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }

    #[test]
    fn string_test() {
        let mut s : String = String::new();//String不用引用，默认在std::路径下

        let data : & str = "123";

        assert_eq!(3, data.len());

        let s : String = data.to_string();

// 该方法也可直接用于字符串字面值：
        let s : String = "initial contents".to_string();

        let s : String = String::from("initial contents");
        assert_eq!(16, s.len());
    }

    #[test]
    fn string_varibale(){
        let mut s = String::from("foo");
        s.push_str("bar");

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);
    }

    #[test]
    fn string_add(){

        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

        println!("s3 is {}", s3);

    }


    #[test]
    fn a(){
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;

        println!("s is {}", s);
    }


    #[test]
    fn test_string_format(){

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("s is {}", s);
    }


    #[test]
    fn abcs(){

    }


    #[test]
    fn asdbc(){

    }




    #[test]
    fn abfdfc(){

    }




    #[test]
    fn abcfdf(){

    }




    #[test]
    fn afdsfdsfsbc(){

    }




    #[test]
    fn abfdsddc(){

    }




    #[test]
    fn abddddsssc(){

    }




    #[test]
    fn abaaaaac(){

    }



    #[test]
    fn afdfdsbc(){

    }



    #[test]
    fn abgfdgsc(){

    }



    #[test]
    fn abgsdffdgc(){

    }



    #[test]
    fn abgsdfgc(){

    }



    #[test]
    fn abgfgsdc(){

    }



    #[test]
    fn absgdfc(){

    }



    #[test]
    fn agdfbc(){

    }








}
