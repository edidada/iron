#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn string_test() {
        let mut s = String::new();//String不用引用，默认在std::路径下

        let data = "initial contents";

        let s = data.to_string();

// 该方法也可直接用于字符串字面值：
        let s = "initial contents".to_string();

        let s = String::from("initial contents");
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
}
